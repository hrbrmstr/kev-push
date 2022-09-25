window.SIDEBAR_ITEMS = {"fn":[["create_kev_cache_file","This is just a wrapper which makes it clearer what we’re creating."],["notify","This uses Pushover to notify when there’s a new KEV release You need to have the `PUSHOVER_USER` (key) and `PUSHOVER_APP` (token) environment variables set in order for the notification to work."],["read_kev_cache_from_file","This is used to read the locally cached KEV JSON file"],["read_kev_from_cisa","This is just a wrapper function which makes it clearer what we’re fetching."],["run","At first launch, this will cache the current KEV JSON. Subsequent launches will then compare the current catalog served from CISA’s site with the cached one and both update the local cache and fire off a notification."]],"struct":[["Kev","This struct enables (de)serialization of the KEV JSON as of 2022-09-24"],["Vulnerability","This struct enables (de)serialization of the KEV JSON as of 2022-09-24"]]};