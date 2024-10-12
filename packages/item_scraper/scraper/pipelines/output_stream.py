import json
import pika
import asyncio

# useful for handling different item types with a single interface
from itemadapter import ItemAdapter
from rstream import Producer
from scrapy.exceptions import DropItem

class OutputStreamPipeline:
    def __init__(self, stream_name):
        self.stream_name = stream_name
        self.connection = None
        self.channel = None

        self.producer = None

    @classmethod
    def from_crawler(cls, crawler):
        return cls(
            stream_name=crawler.settings.get('RABBITMQ_QUEUE')
        )

    def open_spider(self, spider):
        async def init_stream():
            async with Producer(
                host="localhost",
                username="guest",
                password="guest"
            ) as producer:
                self.producer = producer
                stream_name = "scraper_stream"
                stream_retention = 5000000000

                await producer.create_stream(
                    stream_name,
                    exists_ok=True,
                    arguments={"MaxLengthBytes": stream_retention}
                )

        asyncio.run(init_stream())

    def close_spider(self, spider):
        return

    def process_item(self, item, spider):
        try:
            async def send_item(item_json):
                await self.producer.send(
                    stream=self.stream_name,
                    message=item_json
                )

            item_json = json.dumps(dict(item))
            asyncio.run(send_item(item_json))
            spider.logger.info(f"Item sent to RabbitMQ queue: {self.stream_name}")
            return item
        
        except Exception as e:
            spider.logger.error(f"Error sending item to RabbitMQ: {str(e)}")
            raise DropItem(f"Error sending item to RabbitMQ: {str(e)}")