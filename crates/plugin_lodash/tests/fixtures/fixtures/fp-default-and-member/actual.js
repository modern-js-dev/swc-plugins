import fp, { map, take } from "lodash/fp";

var mapper = map(fp.add(1));
var result = mapper([1, 2, 3]);
take(1, fp.reject(Boolean, result));
