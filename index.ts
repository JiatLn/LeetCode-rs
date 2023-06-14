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
  const fileName = questionName.replaceAll("-", "_")
  await mkdir(`src/${fileName}`)
  const file = `src/${fileName}/mod.rs`
  await writeFile(file, rsTemplate(questionLink, fileName))
  console.log(`文件创建成功 ${file}`)
  console.log(`[TODO] 前往 src/main.rs 中添加 mod ${fileName};`)
} catch (err) {
  console.error(err.message);
}

function rsTemplate(questionLink: string, modName: string) {
  return `/// <${questionLink}>
struct Solution;

impl Solution {

}

#[cfg(test)]
mod tests {
    use crate::${modName}::Solution;

    #[test]
    fn test_() {
        
    }
}

`
}
