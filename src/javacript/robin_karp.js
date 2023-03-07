function findNeedle(haystack, needle) {
    const haystackLength = haystack.length;
    const needleLength = needle.length;
    
    if (needleLength === 0) {
      return 0;
    }
    
    for (let i = 0; i < haystackLength; i++) {
      if (haystack[i] === needle[0]) {
        let matched = true;
        
        for (let j = 1; j < needleLength; j++) {
          if (haystack[i+j] !== needle[j]) {
            matched = false;
            break;
          }
        }
        
        if (matched) {
          return i;
        }
      }
    }
    
    return -1;
  }
  
  // Example usage:
  console.log(findNeedle("sadbutsad", "sad")); // Output: 0
  console.log(findNeedle("leetcode", "leeto")); // Output: -1