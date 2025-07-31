#if !defined(TRIANGLE_H)
#define TRIANGLE_H

namespace triangle {

    
    int kind(double a, double b, double c);
    void check(double a, double b, double c);
    namespace flavor{
        const int equilateral {1};
        const int isosceles   {2};
        const int scalene    {3};
    }
    
}  // namespace triangle

#endif // TRIANGLE_H