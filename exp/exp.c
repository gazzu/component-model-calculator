#include "exponential.h"

uint32_t exports_docs_calculator_exp_exp(uint32_t base, uint32_t n) {
	uint32_t i, p;
    p = 1;

    for (i=1; i<=n; ++i) {
        p = p*base;
    }

    return p;
}