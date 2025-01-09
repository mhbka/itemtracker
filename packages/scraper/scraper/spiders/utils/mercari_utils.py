import json
from cryptography.hazmat.primitives.asymmetric import ec
from scraper.spiders.utils.generate_dpop import generate_dpop

# Generate headers for accessing Mercari APIs
def gen_headers(private_key: ec.EllipticCurvePrivateKey, accessed_url: str, http_method: str):
    return {
        "content-type": "application/json",
        "dpop": generate_dpop(private_key, accessed_url, http_method),
        "x-platform": "web"
    }

# Generate payload JSON string for hitting Mercari search API
def gen_payload_string(page_token, search_criteria, logger=None):
    payload = {
        "userId": "",
        "pageSize": 120,
        "pageToken": page_token,
        "searchSessionId": "adc97d31b66ba64443fe25778dee77c2",
        "indexRouting": "INDEX_ROUTING_UNSPECIFIED",
        "thumbnailTypes": [],
        "searchCondition": {
            "keyword": search_criteria.get("keyword", ""),
            "excludeKeyword": search_criteria.get("excludeKeyword", ""),
            "sort": "SORT_CREATED_TIME", # Both `sort` and `order` can have other values, but we should always be scraping for the newest items,
            "order": "ORDER_DESC", # so these are hardcoded to only get the newest items. 
            "status": search_criteria.get("status", []),
            "sizeId": [],
            "categoryId": search_criteria.get("categoryId", []),
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
        "withSearchConditionId": False,
        "withAuction": False,
    }
    payload_str = json.dumps(payload)
    if logger: logger.info(f"SEARCH PAYLOAD: {payload_str}")
    return payload_str

