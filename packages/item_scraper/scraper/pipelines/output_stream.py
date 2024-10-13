import json
import asyncio
from itemadapter import ItemAdapter
from rstream import Producer
from scrapy.exceptions import DropItem
from scrapy.utils.serialize import ScrapyJSONEncoder
from twisted.internet import defer

class OutputStreamPipeline:
    def __init__(self, stream_name, stream_host, stream_username, stream_password):
        self.json_encoder = ScrapyJSONEncoder()
        self.stream_name = stream_name
        self.stream_host = stream_host
        self.stream_username = stream_username
        self.stream_password = stream_password
        self.producer = None

    @classmethod
    def from_crawler(cls, crawler):
        return cls(
            stream_name=crawler.settings.get('OUTPUT_STREAM_NAME'),
            stream_host=crawler.settings.get('OUTPUT_STREAM_HOST'),
            stream_username=crawler.settings.get('OUTPUT_STREAM_USERNAME'),
            stream_password=crawler.settings.get('OUTPUT_STREAM_PASSWORD'),
        )

    def open_spider(self, spider):
        loop = asyncio.get_event_loop()
        loop.create_task(self.init_stream())
        spider.logger.info("Output stream initialized")

    def close_spider(self, spider):
        return

    def process_item(self, item, spider):
        item_json = self.json_encoder.encode({'gallery_id': spider.gallery_id, 'data': item})
        loop = asyncio.get_event_loop()
        loop.create_task(self.send_item(item_json))
        spider.logger.info(f"{item['type']} item {item['id']} for gallery {spider.gallery_id} sent to output stream")
        return item
    
    ## Async stream fns

    async def init_stream(self):
        self.producer = Producer(
            host=self.stream_host,
            username=self.stream_username,
            password=self.stream_password
        )
        await self.producer.create_stream(
            stream=self.stream_name,
            exists_ok=True,
            arguments={"max-length-bytes": 5000000000}
        )
    
    async def send_item(self, item_json):
        await self.producer.send(
            stream = self.stream_name,
            message = item_json.encode('UTF-8')
        )
