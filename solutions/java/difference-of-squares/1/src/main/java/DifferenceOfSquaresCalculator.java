class DifferenceOfSquaresCalculator {

    int computeSquareOfSumTo(int input) {
        int val = input*(input+1)/2;
        return val * val;
    }

    int computeSumOfSquaresTo(int input) {
        int sums = 0;
        for(int i=1; i<=input; i++){
            sums += (i*i);
        }
        return sums;
    }

    int computeDifferenceOfSquares(int input) {
        return computeSquareOfSumTo(input) - computeSumOfSquaresTo(input);
    }

}
