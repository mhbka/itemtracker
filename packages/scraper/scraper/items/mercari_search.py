import scrapy

class MercariSearchItem(scrapy.Item):
    id = scrapy.Field()
    updated = scrapy.Field()