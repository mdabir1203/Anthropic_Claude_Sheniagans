# 42London-Hackathon-Learning


Learning Outcomes : 

## 

## 1.  Prompting Techniques in Claude API : 

- Claude uses <xml> documents underneath 
- XML syntax is used to save promnpt responses to desired variable 
- {} / {{}} are variable parts of strings for insertion . Almost like a substitute variable used in Bash. 
  Used as a regex substitution where the syntax is used for : {{strings to be replaced, strings to be inserted}}

## Prompt sampling :
``` 
choice = "Serious and Professional", "Casual and Fun"
completion = get_completion(CLIENT, 
    f"""\n\nHuman: You are a tutor named Jack&Jones that specializes in Personalized Teaching Asssitant as the given {{subject}}. You are patient, helpful and make learning fun with occasional jokes. You have a knack for creating multiple choice quizzes, precise guideline and can provide tips on any aspect of the topic with rigor.
All of your responses and everything you know about is captured in the {text} document. The {text} is an extract from python code syntax. 

Now generate me a quiz based on the given {text}. Choose the below output format for the quiz:

1. With <quiz> Your Visual Python Learner </>quiz intro you should ask the user if they want to begin.
2. A quiz consists of 5 questions all based on at least one highlight.
3. Question format would be according to user choice : {choice}.
4. The quiz should have an intro that explains that there are 5 questions about Cultural Differences 
5. Each question should have text for the question, four answer options (only one of which is the correct answer) each should have a letter from a to d next to the text for that answer option. The letter should have a dash on the right side of it to separate it from the text answer. Each question should have a response that confirms whether the answer was correct or not and provide the answer coupled with the letter that was used with it.
 
Assistant: Can I think step-by-step and like a rubber duck?

Human: Yes, please do.

+Assistant:


Human: Make an explanation in layman terms of the given answer.

+Assistant:


"""
)
print(completion)
``` 

## 2.  Prompting Post Processing 
https://github.com/webgptorg/promptbook/blob/main/src%2Futils%2Fmarkdown%2FextractOneBlockFromMarkdown.test.ts


```
import { describe, expect, it } from '@jest/globals';
import spaceTrim from 'spacetrim';
import { extractOneBlockFromMarkdown } from './extractOneBlockFromMarkdown';

describe('how extractOneBlockFromMarkdown works', () => {
    it('should work with sample with one unknown code block of one line', () => {
        expect(
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    # Hello World

                    \`\`\`
                    print('Hello World')
                    \`\`\`
                `),
            ),
        ).toEqual({
            language: null,
            content: "print('Hello World')",
        });
    });

    it('should work with sample with one python code block of one line', () => {
        expect(
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    \`\`\`python
                    print('Hello World')
                    \`\`\`
                `),
            ),
        ).toEqual({
            language: 'python',
            content: "print('Hello World')",
        });

        expect(
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    # Hello World

                    \`\`\`python
                    print('Hello World')
                    \`\`\`
                `),
            ),
        ).toEqual({
            language: 'python',
            content: "print('Hello World')",
        });
    });

    it('should work with codeblock with escaped embeded codeblock as content', () => {
        expect(
            extractOneBlockFromMarkdown(
                spaceTrim(`

                  This is a simple markdown with code block with escaped embeded code block as content:

                  \`\`\`markdown

                  Markdown has simple formatting like **bold** and *italic* text.

                  Also it has code blocks:
                  \\\`\\\`\\\`python
                  print('Hello World')
                  \\\`\\\`\\\`

                  And loooot of other features.

                  \`\`\`
              `),
            ),
        ).toEqual({
            language: 'markdown',
            content:
                spaceTrim(`

                  Markdown has simple formatting like **bold** and *italic* text.

                  Also it has code blocks:
                  \`\`\`python
                  print('Hello World')
                  \`\`\`

                  And loooot of other features.

                `) + '\n',
        });
    });

    it('should fail with sample with no code blocks', () => {
        expect(() =>
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    Hello World
                `),
            ),
        ).toThrowError(/There should be exactly one code block in the markdown/i);

        expect(() =>
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    Hello World
                    Hello World
                `),
            ),
        ).toThrowError(/There should be exactly one code block in the markdown/i);

        expect(() =>
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    # Hello World

                    Content with **bold** and *italic* text
                `),
            ),
        ).toThrowError(/There should be exactly one code block in the markdown/i);

        expect(() =>
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    # Prague

                    Prague is the capital of Czech Republic. It is a beautiful city. In my opinion, it is the most beautiful city in the world.

                    If you are in Prague, you should visit the following places:

                    - Prague Castle
                    - Charles Bridge
                    - Old Town Square

                    ## Prague Castle

                    \`Prague Castle\` is the largest ancient castle in the world. It is located in the center of Prague.

                    ## Lennon Wall

                    \`Lennon Wall\` is a wall in Prague. It is located in the center of Prague. On this wall, you can see many graffiti like %#2/*\`\`\`7#^
                `),
            ),
        ).toThrowError(/There should be exactly one code block in the markdown/i);
    });

    it('should fail with sample with multiple code blocks of one line', () => {
        expect(() =>
            extractOneBlockFromMarkdown(
                spaceTrim(`
                    # Hello World

                    Hello World in multiple languages:

                    ## Python

                    Hello World in Python:

                    \`\`\`python
                    print('Hello World')
                    \`\`\`

                    ## Javascript

                    Hello World in Javascript:

                    \`\`\`javascript
                    console.info('Hello World')
                    \`\`\`
                `),
            ),
        ).toThrowError(/There should be exactly one code block in the markdown/i);
    });
});

``` 
