
FROM redis

WORKDIR /modules

COPY ./dist/ ./

CMD ["redis-server", "--loadmodule", "/modules/librejson.so", "--loadmodule", "/modules/redisearch.so"]
