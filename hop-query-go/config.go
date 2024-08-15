package main

import (
	"context"
	"log/slog"
	"os"

	"github.com/spf13/viper"
)

type Config struct {
	Database struct {
		Host     string
		Port     int
		User     string
		Password string
	}

	Server struct {
		Host string
		Port string
	}
}

func newConfig(ctx context.Context, logger *slog.Logger) (*Config, error) {
	env := os.Getenv("GO_ENV")
	if env == "" {
		env = "development"
	}

	viper.SetConfigName(env)
	viper.SetConfigType("toml")
	viper.AddConfigPath("./config/")

	if err := viper.ReadInConfig(); err != nil {
		logger.ErrorContext(ctx, "could not read in configuratoin", "err", err)
		return nil, err
	}

	var config Config

	if err := viper.Unmarshal(&config); err != nil {
		logger.ErrorContext(ctx, "could not deserialize configuration", "err", err)
		return nil, err
	}

	return &config, nil
}
