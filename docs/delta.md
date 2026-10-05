* sort by size, descending order so that later(usually big) object can be read directly, while older object require delta expansion.


## Encoding
* variable-length encoding
### Copy Operation
* little endian
* 1-byte header
  * first bit represents type of operation(copy/insert)
  * 4 bits bitmapping offset
  * 3 bits bitmapping size
* 4 bytes representing offset
* 3 bytes representing size

#### Example  
$data_{16} = 00\ 00\ 00\ aa\ 00\ bb\ cc$  
$header_2 = 1\ 0001\ 011$  
$packed\ data = 10001011_2\ cc_{16}\ bb_{16}\ aa_{16}$  

### Insert Operation
* 1-byte header
  * first bit represents type
  * 7 bits represent number of bytes representing the data
