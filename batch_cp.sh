#!/bin/bash

# 复制文件
# cp target/rustcc_6*.json ../notion-sdk-js/examples/intro-to-notion-api/intermediate/

# 更改文件名
for i in {61..66}; do
	cp target/rustcc_$i.json ../notion-sdk-js/examples/intro-to-notion-api/intermediate/
done
