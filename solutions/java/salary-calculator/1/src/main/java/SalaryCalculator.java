public class SalaryCalculator {
    public double salaryMultiplier(int daysSkipped) {
        return daysSkipped > 4 ? .85 : 1.0;
    }

    public int bonusMultiplier(int productsSold) {
        return productsSold > 19 ? 13 : 10;
    }

    public double bonusForProductsSold(int productsSold) {
        return bonusMultiplier(productsSold) * productsSold;
    }

    public double finalSalary(int daysSkipped, int productsSold) {
        double fSal =  1000.0 * salaryMultiplier(daysSkipped) + bonusForProductsSold(productsSold);
        return fSal <= 2000.0 ? fSal : 2000.0;  
    } 
}
