package main

import (
	"context"
	"fmt"
	"sync"

	"github.com/redis/go-redis/v9"
)

type RedisHandler struct{ 
	client redis.Client
}

func NewRedisHandler(addr string) *RedisHandler {
	client := redis.NewClient(&redis.Options{
		Addr:     addr,
		Password: "", // no password set
		DB:       0,  // use default DB
	})
	return &RedisHandler{
		client: *client,
	}
}

func (rh *RedisHandler)Write(ctx context.Context, wg *sync.WaitGroup, key string, data *[]byte) error {
	if wg != nil {
		defer wg.Done()
	} else {
		fmt.Println(rh.client.DBSize(ctx))
	}

	dataString := string(*data)

	err := rh.client.Set(ctx, key, dataString, 0).Err()
	if err != nil {
		return err
	}
	return nil
}
func (rh *RedisHandler)Keys(ctx context.Context, keyPattern string) []string {
	return rh.client.Keys(ctx, keyPattern).Val()
}

func (rh *RedisHandler)DBSize(ctx context.Context) int64 {
	return rh.client.DBSize(ctx).Val()
}