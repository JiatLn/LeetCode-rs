import { mkdir, writeFile, readFile } from 'node:fs/promises';
import { argv, exit } from "node:process"



async function addQuestionTemplate(link: string, question: string) {
  try {
    await mkdir(`src/${question}`)
    const path = `src/${question}/mod.rs`
    await writeFile(path, rsTemplate(link, question), {
      encoding: 'utf-8',
      flag: 'w+'
    })
    console.log(`文件创建成功 ${path}`)
  } catch (err) {
    console.error(err.message);
  }
}

function rsTemplate(link: string, modName: string) {
  return `/// <${link}>
struct Solution;

impl Solution {

}

#[cfg(test)]
mod tests {
    use crate::${modName}::Solution;

    #[test]
    fn test_() {
        let ans = Solution::
        assert_eq!(ans, );
    }
}

`
}

async function addMainMod(question: string) {
  try {
    const originalContent = await readFile('src/main.rs', 'utf-8');
    let lines = originalContent.split('\n');
    lines.pop();
    lines.pop();
    lines.pop();
    lines.push(`mod ${question};`, '', 'fn main() {}', '')
    const modifiedContent = lines.join('\n');
    await writeFile('src/main.rs', modifiedContent, {
      encoding: 'utf-8',
      flag: 'w+'
    });
  } catch (error) {
    console.error(error);
  }
}


async function flow() {
  if (argv.length < 3) {
    console.error("缺少参数 [题目链接]")
    exit(1)
  }

  const questionLink = argv[2]
  console.log('题目链接', questionLink)

  const question = questionLink
    .split("/")
    .filter(Boolean)
    .at(-1)
    .replaceAll("-", "_")

  try {
    await addQuestionTemplate(questionLink, question)
    await addMainMod(question)
  } catch (error) {
    console.error('error', error)
    exit(1)
  }
}

flow()
