import json
import requests
from scrapy.exceptions import NotConfigured
from scraper.items.mercari_item import MercariItem

"""
This pipeline is for collecting and outputting MercariItems to the appropriate API.
"""
class MercariItemsPipeline:
    """
    Initialize the pipeline with configuration parameters:
        `output_host`: The output host
        `output_endpoint`: The output endpoint
    """
    def __init__(self, output_host=None, output_endpoint=None):
        if not all([output_host, output_endpoint]):
            raise NotConfigured("Must specify `output_host` and `output_endpoint`")
        self.output_host = output_host
        self.output_endpoint = output_endpoint
        self.collected_items = []

    """
    Initialize the pipeline from Scrapy crawler settings.
    """
    @classmethod
    def from_crawler(cls, crawler):
        return cls(
            output_host = crawler.settings.get('OUTPUT_HOST'),
            output_endpoint = crawler.settings.get('OUTPUT_MERCARI_ITEMS_ENDPOINT'),
        )

    """
    Check if item matches target type and collect it if so.
    """
    def process_item(self, item, spider):
        if type(item) is MercariItem:
            self.collected_items.append(item)
            spider.logger.info(f"Collected item of ID {item['id']}")
        return item
    
    """
    Send all collected items to the API when spider closes.
    """
    def close_spider(self, spider):
        if not self.collected_items:
            spider.logger.info("No items collected to submit")
            return
        try:
            payload = {
                'gallery_id': spider.gallery_id,
                'items': self.collected_items,
            }
            response = requests.post(
                f"{self.output_host}/{self.output_endpoint}", 
                data=json.dumps(payload),
                timeout=10
            )
            response.raise_for_status()  # Raises exception for bad status codes
            spider.logger.info(f"Successfully submitted {len(self.collected_items)} MercariItems")
        except requests.RequestException as e:
            spider.logger.error(f"Failed to submit collected items: {e}")