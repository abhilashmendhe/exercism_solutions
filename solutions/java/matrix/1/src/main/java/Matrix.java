
class Matrix {

    private int [][]matrix;
    Matrix(String matrixAsString) {
    	String []temp = matrixAsString.split("\n");
    	int len = temp.length;
    	if(len == 1) {
    		matrix = new int[len][len];
    		matrix[0][0] = Integer.parseInt(matrixAsString);
    	} else {
    		String []row = temp[0].split(" ");
    		int rowLen = row.length;
    		matrix = new int[len][rowLen];
    		for(int i=0; i<len; i++) {
    			String []tempRow = temp[i].split(" ");
    			for(int j=0; j<rowLen; j++) {
    				matrix[i][j] = Integer.parseInt(tempRow[j]);
    			}
    		}
    	}
    	
    	
    }

    int[] getRow(int rowNumber) {
    	int numCols = matrix[0].length;
    	int []arr = new int[numCols];
    	for(int i=0; i<numCols; i++) {
    		arr[i] = matrix[rowNumber-1][i];
    	}
    	return arr;
    }

    int[] getColumn(int columnNumber) {
    	int numRows = matrix.length;
    	int []arr = new int[numRows];
    	System.out.println(arr.length);
    	for(int i=0; i<numRows; i++) {
    		arr[i] = matrix[i][columnNumber-1];
    	}
    	return arr;
    }

}
