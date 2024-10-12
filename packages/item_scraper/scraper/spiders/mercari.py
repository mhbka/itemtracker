import json
import scrapy
from item_scraper.scraper.items.mercari import CategoryItem, MercariItem, SellerItem, ShippingItem
from utils.generate_dpop import generate_private_key
from utils.mercari_utils import gen_payload, gen_headers

class MercariSpider(scrapy.Spider):
    name = "mercari"
    search_url = "https://api.mercari.jp/v2/entities:search"
    item_url = "https://api.mercari.jp/items/get"
    dpop_private_key = generate_private_key()

    def start_requests(self):
        yield scrapy.Request(
            self.search_url, 
            method='POST', 
            body=json.dumps(gen_payload('')), 
            headers=gen_headers(self.dpop_private_key, self.search_url, 'POST')
            )

    def parse(self, response):
        data = json.loads(response.text)    
        for item in data['items']:
                print(f'Parsing {item['id']}...')
                yield self.call_parse_item(item)
        
        if data['meta']['nextPageToken']:
            print(f'Parsing next page ({data['meta']['nextPageToken']})...')
            yield scrapy.Request(
                self.search_url,
                method='POST', 
                body=json.dumps(gen_payload(data['meta']['nextPageToken'])), 
                headers=gen_headers(self.dpop_private_key, self.search_url, 'POST')
                )
    
    def call_parse_item(self, item):
        item_url = self.item_url + '?id=' + item['id']
        return scrapy.Request(
            url=item_url, 
            method='GET', 
            headers=gen_headers(self.dpop_private_key, self.item_url, 'GET'), 
            callback=self.parse_item
            )

    def parse_item(self, item_response):
        json_data = json.loads(item_response.text)['data']
        item = MercariItem()

        item['type'] = 'Mercari'
        
        item['id'] = self.safe_get(json_data, 'id')
        item['name'] = self.safe_get(json_data, 'name')
        item['price'] = self.safe_get(json_data, 'price')
        item['description'] = self.safe_get(json_data, 'description')
        item['status'] = self.safe_get(json_data, 'status')

        seller_data = self.safe_get(json_data, 'seller')
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
        
        category_data = self.safe_get(json_data, 'item_category')
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
            payer=self.safe_get(json_data, 'shipping_payer', 'name'),
            method=self.safe_get(json_data, 'shipping_method', 'name'),
            from_area=self.safe_get(json_data, 'shipping_from_area', 'name'),
            duration=self.safe_get(json_data, 'shipping_duration', 'name')
        )
        
        item['photos'] = self.safe_get(json_data, 'photos')
        item['thumbnails'] = self.safe_get(json_data, 'thumbnails')
        item['item_condition'] = self.safe_get(json_data, 'item_condition')
        item['item_size'] = self.safe_get(json_data, 'item_size')
        item['colors'] = self.safe_get(json_data, 'colors')
        item['num_likes'] = self.safe_get(json_data, 'num_likes')
        item['num_comments'] = self.safe_get(json_data, 'num_comments')
        item['created'] = self.safe_get(json_data, 'created')
        item['updated'] = self.safe_get(json_data, 'updated')
        item['is_anonymous_shipping'] = self.safe_get(json_data, 'is_anonymous_shipping')
        item['is_offerable'] = self.safe_get(json_data, 'is_offerable')
        
        yield item

    def safe_get(self, data, *keys):
        for key in keys:
            try:
                data = data[key]
            except KeyError:
                self.logger.warning(f"Key '{key}' is missing from Mercari item")
                return None
        return data
