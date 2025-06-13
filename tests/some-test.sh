set -eux

curl 'https://l22.lc:3012/api/v1/chnot' \
    -X 'PUT' \
    -H 'accept: application/json, text/plain, */*' \
    -H 'accept-language: zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7' \
    -H 'content-type: application/json' \
    -H 'dnt: 1' \
    -H 'k-kspace: public' \
    -H 'k-mkspaces;' \
    -H 'origin: http://l22.lc:3000' \
    -H 'priority: u=1, i' \
    -H 'referer: http://l22.lc:3000/' \
    -H 'sec-ch-ua: "Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24"' \
    -H 'sec-ch-ua-mobile: ?0' \
    -H 'sec-ch-ua-platform: "Windows"' \
    -H 'sec-fetch-dest: empty' \
    -H 'sec-fetch-mode: cors' \
    -H 'sec-fetch-site: cross-site' \
    -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36' \
    --data-raw '{"content":"fasdfasdf","kind":"mdwt"}' \
    --insecure

curl 'https://l22.lc:3012/api/v1/chnot-query' \
    -H 'accept: application/json, text/plain, */*' \
    -H 'accept-language: zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7' \
    -H 'content-type: application/json' \
    -H 'dnt: 1' \
    -H 'k-kspace: public' \
    -H 'k-mkspaces;' \
    -H 'origin: http://l22.lc:3000' \
    -H 'priority: u=1, i' \
    -H 'referer: http://l22.lc:3000/' \
    -H 'sec-ch-ua: "Google Chrome";v="137", "Chromium";v="137", "Not/A)Brand";v="24"' \
    -H 'sec-ch-ua-mobile: ?0' \
    -H 'sec-ch-ua-platform: "Windows"' \
    -H 'sec-fetch-dest: empty' \
    -H 'sec-fetch-mode: cors' \
    -H 'sec-fetch-site: cross-site' \
    -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36' \
    --data-raw '{"start_index":0,"page_size":20,"query":"s","view_type":{"kind":"timeline"},"kinds":[]}' \
    --insecure

