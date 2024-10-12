```mermaid
    graph TD
        %% Color definitions
        classDef service fill:#E6F3E6,stroke:#000,stroke-width:2px,color:#000;
        classDef queue fill:#FFE4B5,stroke:#000,stroke-width:2px,color:#000;
        classDef database fill:#E6F3FF,stroke:#000,stroke-width:2px,color:#000;
        classDef external fill:#F0E6FF,stroke:#000,stroke-width:2px,color:#000;
        classDef user fill:#FFE4E1,stroke:#000,stroke-width:2px,color:#000;

        S[User]:::user
        P[Web/Mobile App]:::external
        Q[Backend Service]:::service
        A[Web Scraper Service]:::service
        B[Multiple Marketplaces]:::external
        C{RabbitMQ Direct}:::queue
        D[Image Analysis Service]:::service
        E[LLM API]:::external
        F{RabbitMQ Direct}:::queue
        G[Image Classifier Service]:::service
        H[Local AI Model]:::external
        I{RabbitMQ Direct}:::queue
        J[Storage Service]:::service
        K[(Database)]:::database
        L{RabbitMQ Topic}:::queue
        M[Notification Service]:::service
        N[User Notification Channels]:::external
        Z[(Vector DB)]:::database

        S -->|Creates/Updates gallery| P
        P -->|Sends gallery preferences| Q
        Q -->|Sends gallery parameters| A
        A -->|Scrapes periodically| B
        A -->|Pushes items with gallery ID| C
        C -->|analysis_queue| D
        D <-.->|Fetches gallery criteria| Q
        D -->|Analyzes images| E
        D -->|Sends relevant/unsure items| F
        F -->|classifier_queue| G
        G -->|Classifies images| H
        G <-.->|Fetches/Updates item embeddings| Z
        G -->|Classified items with IDs| I
        I -->|storage_queue| J
        J -->|Writes/Reads data| K
        J -->|Publishes updates| L
        L -->|update_exchange| M
        M -->|Sends notifications| N
        Q <-.->|Queries/Updates| J
```