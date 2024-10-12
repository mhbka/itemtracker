from cryptography.hazmat.primitives.asymmetric import ec
from scraper.spiders.utils.generate_dpop import generate_dpop

def gen_payload(page_token: str):
    return {
        "userId": "",
        "pageSize": 120,
        "pageToken": page_token,
        "searchSessionId": "adc97d31b66ba64443fe25778dee77c2",
        "indexRouting": "INDEX_ROUTING_UNSPECIFIED",
        "thumbnailTypes": [],
        "searchCondition": {
            "keyword": "kanon shirt",
            "excludeKeyword": "",
            "sort": "SORT_SCORE",
            "order": "ORDER_DESC",
            "status": ["STATUS_ON_SALE"],
            "sizeId": [],
            "categoryId": [30],
            "brandId": [],
            "sellerId": [],
            "priceMin": 0,
            "priceMax": 0,
            "itemConditionId": [],
            "shippingPayerId": [],
            "shippingFromArea": [],
            "shippingMethod": [],
            "colorId": [],
            "hasCoupon": False,
            "attributes": [],
            "itemTypes": [],
            "skuIds": [],
            "shopIds": []
        },
        "defaultDatasets": [
            "DATASET_TYPE_MERCARI",
            "DATASET_TYPE_BEYOND"
        ],
        "serviceFrom": "suruga",
        "withItemBrand": True,
        "withItemSize": False,
        "withItemPromotions": True,
        "withItemSizes": True,
        "withShopname": False,
        "useDynamicAttribute": True,
        "withSuggestedItems": True,
        "withOfferPricePromotion": False,
        "withProductSuggest": True,
        "withParentProducts": False,
        "withProductArticles": False,
        "withSearchConditionId": False
    }

def gen_headers(private_key: ec.EllipticCurvePrivateKey, accessed_url: str, http_method: str):
    return {
        "content-type": "application/json",
        "dpop": generate_dpop(private_key, accessed_url, http_method),
        "x-platform": "web"
    }
