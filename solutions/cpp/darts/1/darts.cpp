#include "darts.h"

namespace darts {
    int score(double x, double y){
        double rad = sqrt(x*x + y*y);
        if(rad <= 1)
            return 10;
        else if(rad <= 5)
            return 5;
        else if (rad <= 10)
            return 1;
        return 0;
    }
} // namespace darts