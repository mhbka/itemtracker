# Build stage
FROM python:3.9 AS build-stage

RUN apt-get update && apt-get install -y g++

RUN pip install --no-cache-dir scrapyd-client pipreqs

WORKDIR /workdir

COPY . .

# Generate requirements.txt
RUN pipreqs .

# Install project dependencies
RUN pip install -r requirements.txt

# Build an egg of your project
RUN scrapyd-deploy --build-egg=myproject.egg

# Final stage
FROM python:3.9-slim

# Install Scrapy dependencies and any others for your project
RUN apt-get update && apt-get install -y \
    gcc \
    libffi-dev \
    libssl-dev \
    libxml2-dev \
    libxslt-dev \
    g++ \
 && pip install --no-cache-dir scrapyd \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# Copy requirements.txt from build stage and install dependencies
COPY --from=build-stage /workdir/requirements.txt .
RUN pip install -r requirements.txt

# Mount two volumes for configuration and runtime
VOLUME /etc/scrapyd/ /var/lib/scrapyd/

COPY ./scrapyd.conf /etc/scrapyd/

RUN mkdir -p /src/eggs/myproject

COPY --from=build-stage /workdir/myproject.egg /src/eggs/myproject/1.egg

EXPOSE 6800

ENTRYPOINT ["scrapyd", "--pidfile="]