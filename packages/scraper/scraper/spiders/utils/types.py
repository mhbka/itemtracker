## Schemas are defined here to validate types passed into search criteria
## for spiders to use.

mercari_search_criteria_schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "properties": {
        "priceMin": {
            "type": "number"
        },
        "priceMax": {
            "type": "number"
        },
        "keyword": {
            "type": "string"
        },
        "excludeKeyword": {
            "type": "string"
        },
        "sort": {
            "type": "string",
            "enum": [
                # TODO: fetch dynamically or leave hardcoded?
                "SORT_SCORE",
                "SORT_CREATED_TIME",
                "SORT_PRICE",
                "SORT_NUM_LIKES"
            ]
        },
        "order": {
            "type": "string",
            "enum": [
                # TODO: fetch dynamically or leave hardcoded?
                "ORDER_DESC",
                "ORDER_ASC"
            ]
        },
        "status": {
            "type": "array",
            "items": {
            "type": "string",
            "enum": [
                # TODO: fetch dynamically or leave hardcoded?
                "STATUS_ON_SALE",
                "STATUS_SOLD_OUT",
                "STATUS_TRADING"
            ]
            },
            "uniqueItems": True
        },
        "categoryId": {
            "type": "array",
            "items": {
                "type": "number"
            }
        }
    },
    "additionalProperties": False
}