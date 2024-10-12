import json
import scrapy
from tutorial.items import CategoryItem, MercariItem, SellerItem, ShippingItem

dpop_search = "eyJ0eXAiOiJkcG9wK2p3dCIsImFsZyI6IkVTMjU2IiwiandrIjp7ImNydiI6IlAtMjU2Iiwia3R5IjoiRUMiLCJ4IjoiTWd4ekljd3ZDVnpKaDBteHJCb1dSZXp3ZzZXY1hiRTVOc3lENTJJb2c5QSIsInkiOiJZN081cWlVdDVYWDJVR2c1X1VtNXFKaVZXblpwUlJGamRQWnJ3NnkySUFBIn19.eyJpYXQiOjE3Mjg1NTUwOTcsImp0aSI6IjJlZDdkZmI2LTQ5MGYtNGQzYy05YTNhLWZjOWY5ODNkNzExNyIsImh0dSI6Imh0dHBzOi8vYXBpLm1lcmNhcmkuanAvdjIvZW50aXRpZXM6c2VhcmNoIiwiaHRtIjoiUE9TVCIsInV1aWQiOiIyYjYxMDYxNi0wMWJmLTRlYmYtOWI1Ni1lM2QxNzUzNTZlMWEifQ.5qVwXJk7UA_YaaCIs6FLhfGhUYnRImi1hTmDjjaZqJG-_mD7H8Ok8s1q39J07KkW6Fkbmq3J9SVHaiyW_J7lZg"
dpop_item = "eyJ0eXAiOiJkcG9wK2p3dCIsImFsZyI6IkVTMjU2IiwiandrIjp7ImNydiI6IlAtMjU2Iiwia3R5IjoiRUMiLCJ4IjoiNWY3SjFqbEVuOGt4OTBKekp2QlBGQndnaWhtTFhKSUVQbGNCdS14QmkzcyIsInkiOiJSMDJQbm4xRWVTbllaQU5Oa2dsVVlvV0t6S2UyUl9iNXpaSFN6S3B1TkRBIn19.eyJpYXQiOjE3Mjg1NjE2NjAsImp0aSI6IjA3ZDE0YzgzLTRjY2UtNDEzMS1hMTIwLTRmNzY3YjM1NzIxOCIsImh0dSI6Imh0dHBzOi8vYXBpLm1lcmNhcmkuanAvaXRlbXMvZ2V0IiwiaHRtIjoiR0VUIiwidXVpZCI6IjIzNTdkNDBlLTVhZmUtNGM3ZS1hZTU0LTRkNjRkMmM4NjNmOCJ9.E22vSx0ObosEIK9smIy0sNwDRqqbZisetVN9OlTjrGAr2VH45-NJQCumt8FcioksUslCR7JnOl-8y9p2Dwh_PA"

def gen_payload(page_token):
    return {
        "userId": "",
        "pageSize": 120,
        "pageToken": page_token,
        "searchSessionId": "adc97d31b66ba64443fe25778dee77c2",
        "indexRouting": "INDEX_ROUTING_UNSPECIFIED",
        "thumbnailTypes": [],
        "searchCondition": {
            "keyword": "kanon shirt",
            "excludeKeyword": "",
            "sort": "SORT_SCORE",
            "order": "ORDER_DESC",
            "status": [],
            "sizeId": [],
            "categoryId": [],
            "brandId": [],
            "sellerId": [],
            "priceMin": 0,
            "priceMax": 0,
            "itemConditionId": [],
            "shippingPayerId": [],
            "shippingFromArea": [],
            "shippingMethod": [],
            "colorId": [],
            "hasCoupon": False,
            "attributes": [],
            "itemTypes": [],
            "skuIds": [],
            "shopIds": []
        },
        "defaultDatasets": [
            "DATASET_TYPE_MERCARI",
            "DATASET_TYPE_BEYOND"
        ],
        "serviceFrom": "suruga",
        "withItemBrand": True,
        "withItemSize": False,
        "withItemPromotions": True,
        "withItemSizes": True,
        "withShopname": False,
        "useDynamicAttribute": True,
        "withSuggestedItems": True,
        "withOfferPricePromotion": False,
        "withProductSuggest": True,
        "withParentProducts": False,
        "withProductArticles": False,
        "withSearchConditionId": False
    }

def gen_headers(dpop_string):
    return {
        "content-type": "application/json",
        "dpop": dpop_string,
        "x-platform": "web"
    }

def gen_item_url(item_id):
    return  f"https://api.mercari.jp/items/get?id={item_id}&include_item_attributes=true&include_product_page_component=true&include_non_ui_item_attributes=true&include_donation=true&include_offer_like_coupon_display=true&include_offer_coupon_display=true&include_item_attributes_sections=false&country_code=SG"

class QuotesSpider(scrapy.Spider):
    name = "mercari"
    cur_page = 1
    search_url = "https://api.mercari.jp/v2/entities:search"

    def start_requests(self):
        yield scrapy.Request(self.search_url, method='POST', 
                        body=json.dumps(gen_payload('')), 
                        headers=gen_headers(dpop_search))

    def parse(self, response):
        data = json.loads(response.text)    
        for item in data['items']:
                print(f'Parsing {item['id']}...')
                yield self.call_parse_item(item)
        
        if data['meta']['nextPageToken']:
            print(f'Parsing next page ({data['meta']['nextPageToken']})...')
            yield scrapy.Request(self.search_url, method='POST', 
                        body=json.dumps(gen_payload(data['meta']['nextPageToken'])), 
                        headers=gen_headers(dpop_search))
    
    def call_parse_item(self, item):
        return scrapy.Request(url=gen_item_url(item['id']), method='GET', headers=gen_headers(dpop_item), callback=self.parse_item)

    def parse_item(self, item_response):
        json_data = json.loads(item_response.text)['data']
        item = MercariItem()
        
        # Populate main fields
        item['id'] = self.safe_get(json_data, 'id')
        item['name'] = self.safe_get(json_data, 'name')
        item['price'] = self.safe_get(json_data, 'price')
        item['description'] = self.safe_get(json_data, 'description')
        item['status'] = self.safe_get(json_data, 'status')
        
        # Populate nested seller field
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
        
        # Populate nested category field
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
        
        # Populate nested shipping field
        item['shipping'] = ShippingItem(
            payer=self.safe_get(json_data, 'shipping_payer', 'name'),
            method=self.safe_get(json_data, 'shipping_method', 'name'),
            from_area=self.safe_get(json_data, 'shipping_from_area', 'name'),
            duration=self.safe_get(json_data, 'shipping_duration', 'name')
        )
        
        # Populate other fields
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
