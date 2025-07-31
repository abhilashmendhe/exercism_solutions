import java.util.HashMap;
import java.util.Map;

class IsogramChecker {

    boolean isIsogram(String phrase) {
        Map<Character, Integer> m = new HashMap<Character, Integer>();
    	
    	for(int i=0; i<phrase.length(); i++) {
    		char c = phrase.charAt(i);
    		if(c > 64 && c < 91)
    			c += 32;
            if(c > 94 && c < 123)
                if(m.get(c)==null)
        			m.put(c, 1);
        		else
        			m.put(c,m.get(c)+1);
    	}
    	
    	for(Map.Entry<Character, Integer> set : m.entrySet()) {
    		if(set.getValue() > 1)
    			return false;
    	}
    	return true;
    }

}
