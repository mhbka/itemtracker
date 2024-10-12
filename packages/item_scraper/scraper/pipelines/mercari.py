from itemadapter import ItemAdapter
from scrapy.exceptions import DropItem
from scraper.items.mercari import MercariItem

class MercariPipeline:
    def __init__(self):
        self.ids_seen = set()

    def process_item(self, item, spider):
        if type(item) is not MercariItem: return item

        return item