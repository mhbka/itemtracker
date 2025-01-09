import scrapy
from itemloaders.processors import MapCompose, Join
from datetime import datetime

def convert_timestamp(timestamp):
    return datetime.fromtimestamp(int(timestamp))

class SellerItem(scrapy.Item):
    id = scrapy.Field()
    name = scrapy.Field()
    photo_url = scrapy.Field()

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
    category = scrapy.Field()
    photos = scrapy.Field()
    thumbnails = scrapy.Field()
    item_condition = scrapy.Field()
    item_size = scrapy.Field()
    brand = scrapy.Field()

    seller = scrapy.Field(serializer=SellerItem)
    shipping = scrapy.Field(serializer=ShippingItem)
    
    num_likes = scrapy.Field()
    num_comments = scrapy.Field()
    created = scrapy.Field(input_processor=MapCompose(convert_timestamp))
    updated = scrapy.Field(input_processor=MapCompose(convert_timestamp))
    
    is_anonymous_shipping = scrapy.Field()
    is_offerable = scrapy.Field()