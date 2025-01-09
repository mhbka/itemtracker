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
                    headers=gen_headers(self.dpop_private_key, self.item_url, 'GET')
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
        item['photos'] = self.safe_get(json_data, id, 'photos')
        item['thumbnails'] = self.safe_get(json_data, id, 'thumbnails')

        condition_data = self.safe_get(json_data, id, 'item_condition')
        if condition_data:
            item['item_condition'] = self.safe_get(condition_data, id, 'name')
        size_data = self.safe_get(json_data, id, 'item_size')
        if condition_data:
            item['item_size'] = self.safe_get(size_data, id, 'name')
        brand_data = self.safe_get(json_data, id, 'item_brand')
        if brand_data:
            item['brand'] = self.safe_get(brand_data, id, 'name') 
        seller_data = self.safe_get(json_data, id, 'seller')
        if seller_data:
            item['seller'] = SellerItem(
                id=str(self.safe_get(seller_data, id, 'id')), # Need to convert from int to str for serialization purposes
                name=self.safe_get(seller_data, id, 'name'),
                photo_url=self.safe_get(seller_data, id, 'photo_url')
            )
        
        category_data = self.safe_get(json_data, id, 'item_category')
        if category_data: 
            item['category'] = self.safe_get(category_data, id, 'name')

        item['shipping'] = ShippingItem(
            payer=self.safe_get(json_data, id, 'shipping_payer'),
            method=self.safe_get(json_data, id, 'shipping_method'),
            from_area=self.safe_get(json_data, id, 'shipping_from_area'),
            duration=self.safe_get(json_data, id, 'shipping_duration')
        )
        
        item['num_likes'] = self.safe_get(json_data, id, 'num_likes')
        item['num_comments'] = self.safe_get(json_data, id, 'num_comments')
        item['created'] = self.safe_get(json_data, id, 'created')
        item['updated'] = self.safe_get(json_data, id, 'updated')
        item['is_anonymous_shipping'] = self.safe_get(json_data, id, 'is_anonymous_shipping')
        item['is_offerable'] = self.safe_get(json_data, id, 'is_offerable')
        
        yield item

    """
    Gets the key `key` from a dict `data`, and warns using item ID `id` if it doesn't exist.
    """
    def safe_get(self, data, id, key):
        try:
            return  data[key]
        except KeyError:
            self.logger.warning(f"Property '{key}' is missing from Mercari item {id}")
            return None
