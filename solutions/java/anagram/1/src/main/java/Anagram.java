import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Anagram {
	
	private String word;
	private int []chars;
	
    public Anagram(String word) {
    	this.word = word.toLowerCase();
    	this.chars = new int[26];
    	
    	for(int i=0; i<word.length(); i++) {
    		int ascii = word.charAt(i) > 64 && word.charAt(i) < 96 ? (int) word.charAt(i) + 32 : word.charAt(i);
    		this.chars[ascii % 26]++;
    	}

    }

    public List<String> match(List<String> candidates) {
        
    	List<String> s = new ArrayList<String>();
    	
    	for(int i=0; i<candidates.size(); i++) {
    		
    		String tempWord = candidates.get(i).toLowerCase();
    		if(!this.word.equals(tempWord) && tempWord.length()==this.word.length()) {
    			
    			int []tempChars = new int[26];
    			for(int j=0; j<tempWord.length(); j++) {
    				int ascii = tempWord.charAt(j) > 64 && tempWord.charAt(j) < 96 ? (int) tempWord.charAt(j) + 32 : tempWord.charAt(j);
    				tempChars[ascii%26]++;
    			}
    			if(Arrays.equals(this.chars, tempChars))
    				s.add(candidates.get(i));		
    		}
    	}
    	
    	return s;
    }
}
