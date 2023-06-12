import { mkdir, writeFile } from 'node:fs/promises';
import { argv, exit } from "node:process"

if (argv.length < 3) {
  console.error("缺少参数 [题目链接]")
  exit(1)
}

const questionLink = argv[2]

console.log('题目链接', questionLink)

let questionName = questionLink.split("/").filter(Boolean).at(-1);

try {
  questionName = questionName.replaceAll("-", "_")
  await mkdir(`src/${questionName}`)
  const file = `src/${questionName}/mod.rs`
  await writeFile(file, `/// <${questionLink}> \nstruct Solution;`)
  console.log(`文件创建成功 ${file}`)
} catch (err) {
  console.error(err.message);
}