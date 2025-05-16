import { REST, Routes, SlashCommandBuilder } from "discord.js";
import fs from "node:fs";
import path from "node:path";
import dotenv from "dotenv";

dotenv.config();

const token = process.env.DISCORD_TOKEN as string;
const clientId = process.env.CLIENT_ID as string;

interface Command {
  data: SlashCommandBuilder;
  execute: (...args: any[]) => Promise<void>;
}

const commands = [];
const foldersPath = path.join(__dirname, "commands");
const commandFolders = fs.readdirSync(foldersPath);

for (const folder of commandFolders) {
  const commandsPath = path.join(foldersPath, folder);
  const commandFiles = fs
    .readdirSync(commandsPath)
    .filter((file) => file.endsWith(".js"));

  for (const file of commandFiles) {
    const filePath = path.join(commandsPath, file);
    const command = require(filePath) as Command;

    if ("data" in command && "execute" in command) {
      commands.push(command.data.toJSON());
    } else {
      console.log(
        `[WARNING] The command at ${filePath} is missing a required "data" or "execute" property.`
      );
    }
  }
}

const rest = new REST().setToken(token);

(async () => {
  try {
    console.log(
      `Started refreshing ${commands.length} application (/) commands.`
    );

    const data = await rest.put(Routes.applicationCommands(clientId), {
      body: commands,
    });

    console.log(
      `Successfully reloaded ${
        (data as any[]).length
      } application (/) commands.`
    );
  } catch (error) {
    console.error(error);
  }
})();
