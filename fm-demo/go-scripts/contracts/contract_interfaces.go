package contracts

import (
	"context"
	"math/big"
)

// Storage represents generalized interactions with the stoarge contract
type Storage interface {
	Get(context.Context) (*big.Int, error)
	Set(context.Context, *big.Int) error
}

type FluxAggregator interface {
	Description(context.Context) (string, error)
}

type VRF interface {
	ProofLength(context.Context) (*big.Int, error)
}
