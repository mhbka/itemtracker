import scrapy
from scrapy.loader.processors import MapCompose, TakeFirst, Join
from datetime import datetime

def convert_timestamp(timestamp):
    return datetime.fromtimestamp(int(timestamp))

class SellerItem(scrapy.Item):
    id = scrapy.Field()
    name = scrapy.Field()
    photo_url = scrapy.Field()
    photo_thumbnail_url = scrapy.Field()
    num_sell_items = scrapy.Field()
    ratings = scrapy.Field()
    score = scrapy.Field()
    is_official = scrapy.Field()
    quick_shipper = scrapy.Field()
    star_rating_score = scrapy.Field()

class CategoryItem(scrapy.Item):
    id = scrapy.Field()
    name = scrapy.Field()
    parent_category_id = scrapy.Field()
    parent_category_name = scrapy.Field()
    root_category_id = scrapy.Field()
    root_category_name = scrapy.Field()

class ShippingItem(scrapy.Item):
    payer = scrapy.Field()
    method = scrapy.Field()
    from_area = scrapy.Field()
    duration = scrapy.Field()

class MercariItem(scrapy.Item):
    id = scrapy.Field()
    name = scrapy.Field()
    price = scrapy.Field()
    description = scrapy.Field()
    status = scrapy.Field()
    
    seller = scrapy.Field(serializer=SellerItem)
    item_category = scrapy.Field(serializer=CategoryItem)
    shipping = scrapy.Field(serializer=ShippingItem)
    
    photos = scrapy.Field()
    thumbnails = scrapy.Field()
    
    item_condition = scrapy.Field()
    item_size = scrapy.Field()
    colors = scrapy.Field(
        input_processor=MapCompose(lambda x: x['name']),
        output_processor=Join(', ')
    )
    
    num_likes = scrapy.Field()
    num_comments = scrapy.Field()
    created = scrapy.Field(input_processor=MapCompose(convert_timestamp))
    updated = scrapy.Field(input_processor=MapCompose(convert_timestamp))
    
    is_anonymous_shipping = scrapy.Field()
    is_offerable = scrapy.Field()