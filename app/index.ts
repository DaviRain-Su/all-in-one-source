import { Client } from "@notionhq/client"
import { config } from "dotenv"
import fs from "fs"

config()

const pageId = process.env.NOTION_PAGE_ID ?? "";
const apiKey = process.env.NOTION_API_KEY ?? "";

const notion = new Client({ auth: apiKey })

/*
---------------------------------------------------------------------------
*/

/**
 * Resources:
 * - Create a database endpoint (notion.databases.create(): https://developers.notion.com/reference/create-a-database)
 * - Create a page endpoint (notion.pages.create(): https://developers.notion.com/reference/post-page)
 * - Working with databases guide: https://developers.notion.com/docs/working-with-databases
 */


type PageProperties = {
  Title: {
    title: {
      text: {
        content: string
      };
      type: string,
    }[],
    type: string;
  },
  // These properties represent columns in the database (i.e. its schema)
  Author: {
    rich_text: {
      text: {
        content: string
      };
      type: string,
    }[];
    type: string;
  },
  Intro: {
    rich_text: {
      text: {
        content: string
      };
      type: string,
    }[];
    type: string;
  },
  Link: {
    rich_text: {
      text: {
        content: string
      };
      type: string,
    }[];
    type: string;
  },
  Time: {
    rich_text: {
      text: {
        content: string
      };
      type: string,
    }[];
    type: string;
  }
};


async function addNotionPageToDatabase(databaseId: string, pageProperties: PageProperties) {

  const newPage = await notion.pages.create({
    parent: {
      database_id: databaseId,
    },
    properties: pageProperties,
  });
  console.log(newPage);
}

async function main() {
  // Create a new database
  const newDatabase = await notion.databases.create({
    parent: {
      type: "page_id",
      page_id: pageId,
    },
    title: [
      {
        type: "text",
        text: {
          content: "Rust日报 by Davirain",
        },
      },
    ],
    properties: {
      "Title": {
        type: "title",
        title: {},
      },
      // These properties represent columns in the database (i.e. its schema)
      "Author": {
        type: "rich_text",
        rich_text: {},
      },
      "Intro": {
        type: "rich_text",
        rich_text: {},
      },
      "Link": {
        type: "rich_text",
        rich_text: {},
      },
      "Time": {
        type: "rich_text",
        rich_text: {},
      }
    },
  })

  // Print the new database's URL. Visit the URL in your browser to see the pages that get created in the next step.
  console.log(newDatabase.id)

  const databaseId = newDatabase.id
  // If there is no ID (if there's an error), return.
  if (!databaseId) return

  console.log("Adding new pages...")
  // 获取所有以 "rustcc_" 开头的 .js 文件列表
  const files = fs.readdirSync("./data/").filter(file => file.startsWith("rustcc_") && file.endsWith(".js"))
  console.log(files)
  for (let i = 0; i < files.length; i++) {
    const propertiesForNewPagesModule = await import(`./data/${files[i]}`);
    const propertiesForNewPages = propertiesForNewPagesModule.propertiesForNewPages;

    // const propertiesForNewPages = require(`./data/${files[i]}`)
    // await addNotionPageToDatabase(databaseId, propertiesForNewPages)
    console.log("index: ", i, "len is ", propertiesForNewPages.length);
    for (let j = 0; j < propertiesForNewPages.length; j++) {
      // Add a few new pages to the database that was just created
      await addNotionPageToDatabase(databaseId, propertiesForNewPages[j])
    }
  }

}

main()
