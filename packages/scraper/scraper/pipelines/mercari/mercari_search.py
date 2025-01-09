import json
import requests
from itemadapter import ItemAdapter
from scrapy.utils.project import get_project_settings
from scrapy.exceptions import CloseSpider
from scraper.items.mercari_search import MercariSearchItem

"""
This pipeline is for collecting and outputting MercariSearchItems to the appropriate API.
"""
class MercariSearchItemsPipeline:
    """
    Initialize the pipeline.
    """
    def __init__(self):
        self.collected_items = []
        self.init_settings()

    """
    Get required values from the crawler settings.
    """
    def init_settings(self):
        try:
            settings = get_project_settings()
            self.output_url = f"{settings.get('OUTPUT_HOST')}{settings.get('MERCARI_SEARCH_ENDPOINT')}"
        except Exception as e:
            raise CloseSpider(f"{self.name}: Unable to fetch `OUTPUT_HOST` and/or `MERCARI_SEARCH_ENDPOINT` from Scrapyd settings")

    """
    Check if item matches target type and collect it if so.
    """
    def process_item(self, item, spider):
        if type(item) is MercariSearchItem:
            self.collected_items.append(item['id'])
            spider.logger.info(f"Pipeline collected item {item['id']}")
        return item
    
    """
    Send all collected items to the API when spider closes.
    """
    def close_spider(self, spider):
        if len(self.collected_items) == 0:
            spider.logger.info("Nothing to submit for Mercari Search")
            return
        try:
            payload = {
                'gallery_id': spider.gallery_id,
                'updated_up_to': spider.updated_up_to,
                'scraped_item_ids': self.collected_items,
                'marketplace': "Mercari"
            }
            response = requests.post(
                self.output_url, 
                json=payload,
                timeout=10
            )
            response.raise_for_status()  # Raises exception for bad status codes
            spider.logger.info(f"Successfully submitted {len(self.collected_items)} MercariSearchItems")
        except requests.RequestException as e:
            spider.logger.error(f"Failed to submit collected items: {e}")