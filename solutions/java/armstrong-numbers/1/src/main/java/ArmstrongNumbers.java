class ArmstrongNumbers {

    boolean isArmstrongNumber(int numberToCheck) {

        int num1 = numberToCheck;
        int c = 0;
        while(num1>0){
            c++;
            num1/=10;
        }

        num1 = numberToCheck;
        int sum = 0;
        while(num1 > 0){
            int mod = num1 % 10;
            int i = 0;
            int val = 1;
            while(i<c){
                val *= mod;
                i++;
            }
            sum += val;
            num1 /= 10;
        }
        return sum==numberToCheck ? true : false;
    }

}
