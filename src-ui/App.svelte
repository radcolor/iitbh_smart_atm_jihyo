<script lang="ts">
  import { Button } from "@/components/ui/button";
  import { IndianRupee, TerminalSquare, Database, Cctv } from "lucide-svelte";
  import CMenu from "@/components/Menu.svelte";
  import "@fontsource/noto-sans/400.css";
  import "@fontsource/noto-sans/500.css";
  import "@fontsource/noto-sans/600.css";
  import "@fontsource/noto-sans/700.css";
  import "@fontsource/noto-sans/800.css";
  import "@fontsource/source-code-pro/600.css";
  import { Textarea } from "@/components/ui/textarea";
  import { Toaster, toast } from "svelte-sonner";
  import { Badge } from "@/components/ui/badge";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Command } from "@tauri-apps/api/shell";

  let is_surrealdb_running: boolean;
  let is_cv_server_running: boolean;
  let is_dispenser_running: boolean;
  let disable_dispenser_kill_btn: boolean;

  let child: any;

  let text = "";
  let element;

  async function check_surreal_db_server_port() {
    const is_running: Promise<boolean> = invoke("is_port_free", {
      port: 8000,
    });
    if (await is_running) {
      console.log("SurrealDB not running");
      is_surrealdb_running = false;
    } else {
      console.log("SurrealDB is running");
      is_surrealdb_running = true;
    }
  }
  async function check_dispenser_server_port() {
    const is_running: Promise<boolean> = invoke("is_port_free", {
      port: 8080,
    });
    if (await is_running) {
      console.log("Dispenser server not running");
      is_dispenser_running = false;
    } else {
      console.log("Dispenser server is running");
      is_dispenser_running = true;
      disable_dispenser_kill_btn = true;
    }
  }
  check_dispenser_server_port();
  check_surreal_db_server_port();

  function getCurrentLocalTime(): string {
    const currentDate = new Date();
    const hours = String(currentDate.getHours()).padStart(2, "0");
    const minutes = String(currentDate.getMinutes()).padStart(2, "0");
    const seconds = String(currentDate.getSeconds()).padStart(2, "0");
    const milliseconds = String(currentDate.getMilliseconds()).padStart(3, "0");

    return `${hours}:${minutes}:${seconds}.${milliseconds}`;
  }

  function removeAnsiEscapeCodes(input: string): string {
    const ansiEscapeRegex = /\x1B\[[0-?]*[ -/]*[@-~]/g;
    const cleanedString = input.replace(ansiEscapeRegex, "");
    return cleanedString;
  }

  async function start_dbcluster_server() {
    if (is_surrealdb_running) {
      child.kill();
      console.log("pid:", child.pid);
    } else {
      const command = new Command("start_dbcluster", [
        "start",
        "--log",
        "trace",
        "--auth",
        "--user",
        "radcolor",
        "--pass",
        "radcolor",
        "file:../../../smartatm.db",
      ]);
      command.on("close", (data) => {
        console.log(
          `command finished with code ${data.code} and signal ${data.signal}`
        );
      });
      command.on("error", (error) =>
        console.error(`command error: "${error}"`)
      );
      command.stdout.on("data", (line) => {
        text +=
          getCurrentLocalTime() +
          ' <span style="color: blue;">[iitbh_smart_atm_dbcluster]</span> ' +
          removeAnsiEscapeCodes(line);
        var textarea = document.getElementById("log_ta");
        textarea.scrollTop = textarea.scrollHeight;
      });
      command.stderr.on("data", (line) => {
        text +=
          getCurrentLocalTime() +
          " [iitbh_smart_atm_dbcluster] " +
          removeAnsiEscapeCodes(line);
        var textarea = document.getElementById("log_ta");
        textarea.scrollTop = textarea.scrollHeight;
      });

      child = await command.spawn();
    }

    is_surrealdb_running = !is_surrealdb_running;
  }
</script>

<div style="padding: 16px;">
  <!-- <CMenu></CMenu> -->
  <h1 style="font-weight: 700;">
    IITBH Smart ATM Jihyo (ATM control panel) v1.4
  </h1>
  <br />
  <div class="flex sm:flex-col px-4 py-3 rounded-md border">
    <div class="flex items-center space-x-4">
      <!-- <Avatar.Root>
        <Avatar.Image src="person.avatar" />
      </Avatar.Root> -->
      <Database />
      <div>
        <p class="text-sm font-medium leading-none">
          Smart ATM Database Instance {#if is_surrealdb_running}<Badge
              variant="outline">running</Badge
            >{/if}
        </p>
        <p class="text-sm text-muted-foreground">
          Database instance for atm terminal for users, transactions, atms etc
        </p>
      </div>
      <div class="flex-grow text-right">
        {#if is_surrealdb_running}
          <Button
            variant="destructive"
            on:click={() => start_dbcluster_server()}
          >
            Stop server
          </Button>
        {:else}
          <Button variant="outline" on:click={() => start_dbcluster_server()}>
            Start server
          </Button>
        {/if}
        <Button disabled variant="secondary">Admin</Button>
      </div>
    </div>
    <br />
    <div class="flex items-center space-x-4">
      <!-- <Avatar.Root>
        <Avatar.Image src="person.avatar" />
      </Avatar.Root> -->
      <Cctv />
      <div>
        <p class="text-sm font-medium leading-none">
          Smart ATM CV Security Instance {#if is_cv_server_running}<Badge
              variant="outline">running</Badge
            >{/if}
        </p>
        <p class="text-sm text-muted-foreground">
          Database instance for atm terminal for users, transactions, atms etc
        </p>
      </div>
      <div class="flex-grow text-right">
        <Button
          disabled
          variant="outline"
          on:click={() =>
            toast.success("Server has been started", {
              description: "Sunday, December 03, 2023 at 9:00 AM",
            })}
        >
          Start server
        </Button>
        <Button disabled variant="secondary">Admin</Button>
      </div>
    </div>
    <br />
    <div class="flex items-center space-x-4">
      <!-- <Avatar.Root>
        <Avatar.Image src="person.avatar" />
      </Avatar.Root> -->
      <IndianRupee />
      <div>
        <p class="text-sm font-medium leading-none">
          Smart ATM Dispenser Instance {#if is_dispenser_running}<Badge
              variant="outline">running</Badge
            >{/if}
        </p>
        <p class="text-sm text-muted-foreground">
          Dispenser instance for terminal for atm transactions and app as well
        </p>
      </div>
      <div class="flex-grow text-right">
        {#if is_dispenser_running}
          <Button
            variant="destructive"
            on:click={() =>
              toast("Server has been started", {
                description: "Sunday, December 03, 2023 at 9:00 AM",
              })}
          >
            Stop/Kill dispenser server
          </Button>
        {:else}
          <Button
            disabled
            variant="destructive"
            on:click={() =>
              toast("Server has been started", {
                description: "Sunday, December 03, 2023 at 9:00 AM",
              })}
          >
            Stop/Kill dispenser server
          </Button>
        {/if}
      </div>
    </div>
    <br />
    <div class="flex items-center space-x-4">
      <!-- <Avatar.Root>
        <Avatar.Image src="person.avatar" />
      </Avatar.Root> -->
      <TerminalSquare />
      <div>
        <p class="text-sm font-medium leading-none">
          Smart ATM Terminal Instance
        </p>
        <p class="text-sm text-muted-foreground">
          Database instance for atm terminal for users, transactions, atms etc
        </p>
      </div>
      <div class="flex-grow text-right">
        {#if !is_dispenser_running && is_surrealdb_running}
          <Button
            variant="outline"
            on:click={() =>
              toast.success("Server has been started", {
                description: "Sunday, December 03, 2023 at 9:00 AM",
              })}
          >
            Start smart ATM terminal
          </Button>
        {:else}
          <Button
            disabled
            variant="outline"
            on:click={() =>
              toast.success("Server has been started", {
                description: "Sunday, December 03, 2023 at 9:00 AM",
              })}
          >
            Start smart ATM terminal
          </Button>
        {/if}
      </div>
    </div>
  </div>
  <br />
  <Textarea
    id="log_ta"
    bind:value={text}
    bind:this={element}
    readonly
    wrap="on"
    class="logcat"
    style="min-height: 200px; max-height: 200px; cursor: default; font-family: 'Source Code Pro', monospace; overflow:auto;"
    disabled
    placeholder="log..."
  />
  <p
    class="text-sm text-muted-foreground"
    style="margin-top: 16px; font-weight: 500;"
  >
    iitbh_smart_atm_jihyo copyright c, IITBH 2023-2024, all rights reserved
  </p>
</div>
<Toaster richColors expand={true} />

<style>
  :root {
    font-family: "Noto Sans";
    overflow: hidden;
  }
  p {
    font-size: 12px;
  }
  :global(button) {
    font-size: 12px !important;
  }
  div {
    user-select: none;
    cursor: default;
  }
  /* :global(.logcat) {
    -ms-overflow-style: none;
    scrollbar-width: none;
    overflow-y: scroll;
  }
  :global(.logcat::-webkit-scrollbar) {
    display: none;
  } */
</style>
