import asyncio

from rstream import Producer

STREAM_NAME = "analysis_queue"
# 5GB
STREAM_RETENTION = 5000000000


async def send():
    async with Producer(
        host="localhost",
        username="scraper",
        password="scraper334",
    ) as producer:
        await producer.create_stream(
            STREAM_NAME, exists_ok=True, arguments={"max-length-bytes": STREAM_RETENTION}
        )

        await producer.send(stream=STREAM_NAME, message=b"Hello, World!")

        print(" [x] Hello, World! message sent")

        input(" [x] Press Enter to close the producer  ...")

with asyncio.Runner() as runner:
    runner.run(send())