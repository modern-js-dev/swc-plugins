import _, { map, take } from "lodash-es";

var result = map([], n => _.add(1, n));
take(_.reject(result), 1);
