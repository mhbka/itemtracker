import json
import scrapy
import jsonschema
from scrapy.exceptions import CloseSpider
from scraper.items.mercari_search import MercariSearchItem
from scraper.spiders.utils.generate_dpop import generate_private_key
from scraper.spiders.utils.mercari_utils import gen_payload_string, gen_headers
from scraper.spiders.utils.types import mercari_search_criteria_schema

"""
This spider is for scraping Mercari search.
It takes (set as class params):
    `gallery_id`: A string to associate the scraped data with.
    `search_criteria`: Some criteria to initiate the search under.
    `up_to`: A UNIX timestamp; data updated up to this datetime should be scraped.

It returns a list of Mercari item IDs.
"""
class MercariSearchSpider(scrapy.Spider):
    name = "mercari_search_spider"
    search_url = "https://api.mercari.jp/v2/entities:search" # TODO: to .env
    dpop_private_key = generate_private_key()

    """
    Built-in function for starting the scrape.
    """
    def start_requests(self):
        if self.gallery_id is None or self.search_criteria is None or self.up_to is None:
            raise CloseSpider(f"{self.name}: missing 'gallery_id', 'search_criteria' or 'up_to'; closing spider...")
        else:
            self.search_criteria = json.loads(self.search_criteria)
            try:
                jsonschema.validate(
                    instance = self.search_criteria,
                    schema = mercari_search_criteria_schema
                )
            except jsonschema.ValidationError:
                raise CloseSpider(f"{self.name}: search criteria has wrong schema; closing spider...")
        yield scrapy.Request(
            self.search_url, 
            method = 'POST', 
            body = gen_payload_string('', self.search_criteria, self.logger), 
            headers = gen_headers(self.dpop_private_key, self.search_url, 'POST')
            )

    """
    Parses items scraped from Mercari search.
    
    If the item's `updated` is passed `self.up_to`, return. Else, parse into a MercariSearchItem and yield it.

    If there's a next page, continue scraping it.
    """
    def parse(self, response):
        data = json.loads(response.text)    
        for item in data['items']:
            item_updated = int(item['updated'])
            if item_updated >= self.up_to:
                self.logger.info(f"{self.name}: Found item ID {item['id']}, updated {item['updated']}...")
                yield self.parse_into_item(item)
            else:
                self.logger.info(f"{self.name}: Found item ID {item['id']}, updated {item['updated']} (passed {self.up_to}). Stopping...")
                return
        
        if data['meta']['nextPageToken']:
            self.logger.info(f"Parsing next page ({data['meta']['nextPageToken']})...")
            yield scrapy.Request(
                self.search_url,
                method = 'POST', 
                body = gen_payload_string(data['meta']['nextPageToken'], self.search_criteria, self.logger), 
                headers = gen_headers(self.dpop_private_key, self.search_url, 'POST')
                )
    
    """
    Parse the raw item into a MercariSearchItem.
    """
    def parse_into_item(self, raw_item):
        item = MercariSearchItem()
        try:
            item['id'] = raw_item['id']
            item['updated'] = raw_item['updated']
            yield item
        except:
            self.logger.warning(f"{self.name}: Item was missing `id` or `updated` field.")