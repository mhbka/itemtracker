import json
import scrapy
from scrapy.exceptions import CloseSpider
from scraper.items.mercari_item import CategoryItem, MercariItem, SellerItem, ShippingItem
from scraper.spiders.utils.generate_dpop import generate_private_key
from scraper.spiders.utils.mercari_utils import gen_headers

"""
This spider is for scraping Mercari items.

It takes (set as class params):
    `gallery_id`: A string to associate the scraped data with.
    `item_ids`: A list of item ID strings to scrape.

It returns detailed item data for each given item ID.
"""
class MercariItemsSpider(scrapy.Spider):
    name = "mercari_items_spider"
    item_url = "https://api.mercari.jp/items/get" # TODO: to .env
    dpop_private_key = generate_private_key()

    """
    Built-in function for starting the scrape.
    """
    def start_requests(self):
        if self.gallery_id is None or self.item_ids is None:
            raise CloseSpider("missing `gallery_id` or `item_ids`; closing spider...")
        else:
            self.item_ids = json.loads(self.item_ids)
            for id in self.item_ids:
                item_url = self.item_url + '?id=' + id
                yield scrapy.Request(
                    url=item_url, 
                    method='GET', 
                    headers=gen_headers(self.dpop_private_key, self.item_url, 'GET'), 
                    callback=self.parse_item
                    )

    """
    Parse each item's data into a MercariItem.
    """
    def parse(self, response):
        json_data = json.loads(response.text)['data']
        id = json_data['id']
        item = MercariItem()
        
        item['id'] = self.safe_get(json_data, id, 'id')
        item['name'] = self.safe_get(json_data, id, 'name')
        item['price'] = self.safe_get(json_data, id, 'price')
        item['description'] = self.safe_get(json_data, id, 'description')
        item['status'] = self.safe_get(json_data, id, 'status')

        seller_data = self.safe_get(json_data, id, 'seller')
        if seller_data:
            item['seller'] = SellerItem(
                id=self.safe_get(seller_data, 'id'),
                name=self.safe_get(seller_data, 'name'),
                photo_url=self.safe_get(seller_data, 'photo_url'),
                photo_thumbnail_url=self.safe_get(seller_data, 'photo_thumbnail_url'),
                num_sell_items=self.safe_get(seller_data, 'num_sell_items'),
                ratings=self.safe_get(seller_data, 'ratings'),
                score=self.safe_get(seller_data, 'score'),
                is_official=self.safe_get(seller_data, 'is_official'),
                quick_shipper=self.safe_get(seller_data, 'quick_shipper'),
                star_rating_score=self.safe_get(seller_data, 'star_rating_score')
            )
        
        category_data = self.safe_get(json_data, id, 'item_category')
        if category_data:
            item['item_category'] = CategoryItem(
                id=self.safe_get(category_data, 'id'),
                name=self.safe_get(category_data, 'name'),
                parent_category_id=self.safe_get(category_data, 'parent_category_id'),
                parent_category_name=self.safe_get(category_data, 'parent_category_name'),
                root_category_id=self.safe_get(category_data, 'root_category_id'),
                root_category_name=self.safe_get(category_data, 'root_category_name')
            )
        
        item['shipping'] = ShippingItem(
            payer=self.safe_get(json_data, id, 'shipping_payer', 'name'),
            method=self.safe_get(json_data, id, 'shipping_method', 'name'),
            from_area=self.safe_get(json_data, id, 'shipping_from_area', 'name'),
            duration=self.safe_get(json_data, id, 'shipping_duration', 'name')
        )
        
        item['photos'] = self.safe_get(json_data, id, 'photos')
        item['thumbnails'] = self.safe_get(json_data, id, 'thumbnails')
        item['item_condition'] = self.safe_get(json_data, id, 'item_condition')
        item['item_size'] = self.safe_get(json_data, id, 'item_size')
        item['colors'] = self.safe_get(json_data, id, 'colors')
        item['num_likes'] = self.safe_get(json_data, id, 'num_likes')
        item['num_comments'] = self.safe_get(json_data, id, 'num_comments')
        item['created'] = self.safe_get(json_data, id, 'created')
        item['updated'] = self.safe_get(json_data, id, 'updated')
        item['is_anonymous_shipping'] = self.safe_get(json_data, id, 'is_anonymous_shipping')
        item['is_offerable'] = self.safe_get(json_data, id, 'is_offerable')
        
        yield item

    def safe_get(self, data, id, *keys):
        for key in keys:
            try:
                data = data[key]
            except KeyError:
                self.logger.warning(f"Property '{key}' is missing from Mercari item {id}")
                return None
        return data
