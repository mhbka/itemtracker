import json
import pika

# useful for handling different item types with a single interface
from itemadapter import ItemAdapter
from scrapy.exceptions import DropItem

class OutputQueuePipeline:
    def __init__(self, amqp_url, queue_name):
        self.amqp_url = amqp_url
        self.queue_name = queue_name
        self.connection = None
        self.channel = None

    @classmethod
    def from_crawler(cls, crawler):
        return cls(
            amqp_url=crawler.settings.get('RABBITMQ_URL'),
            queue_name=crawler.settings.get('RABBITMQ_QUEUE', 'scraper_stream')
        )

    def open_spider(self, spider):
        self.connection = pika.BlockingConnection(pika.URLParameters(self.amqp_url))
        self.channel = self.connection.channel()
        self.channel.queue_declare(queue=self.queue_name, durable=True)

    def close_spider(self, spider):
        if self.connection and self.connection.is_open:
            self.connection.close()

    def process_item(self, item, spider):
        try:
            # Convert item to dictionary
            item_dict = dict(item)

            item_type = item_dict.get('name', 'unknown')
            item_dict['type'] = item_type
            item_json = json.dumps(item_dict)
            
            # Publish the item to RabbitMQ
            self.channel.basic_publish(
                exchange='',
                routing_key=self.queue_name,
                body=item_json,
                properties=pika.BasicProperties(
                    delivery_mode=2,  # make message persistent
                )
            )
            spider.logger.info(f"Item sent to RabbitMQ queue: {self.queue_name}")
            return item
        except Exception as e:
            spider.logger.error(f"Error sending item to RabbitMQ: {str(e)}")
            raise DropItem(f"Error sending item to RabbitMQ: {str(e)}")
