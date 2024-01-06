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
          Smart ATM Database Instance
        </p>
        <p class="text-sm text-muted-foreground">
          Database instance for atm terminal for users, transactions, atms etc
        </p>
      </div>
      <div class="flex-grow text-right">
        <Button
          variant="outline"
          on:click={() =>
            toast.success("Server has been started", {
              description: "Sunday, December 03, 2023 at 9:00 AM",
            })}
        >
          Start server
        </Button>
        <Button variant="secondary">Admin</Button>
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
          Smart ATM CV Security Instance
        </p>
        <p class="text-sm text-muted-foreground">
          Database instance for atm terminal for users, transactions, atms etc
        </p>
      </div>
      <div class="flex-grow text-right">
        <Button
          variant="outline"
          on:click={() =>
            toast.success("Server has been started", {
              description: "Sunday, December 03, 2023 at 9:00 AM",
            })}
        >
          Start server
        </Button>
        <Button variant="secondary">Admin</Button>
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
          Smart ATM Dispenser Instance <Badge variant="outline">running</Badge>
        </p>
        <p class="text-sm text-muted-foreground">
          Dispenser instance for terminal for atm transactions and app as well
        </p>
      </div>
      <div class="flex-grow text-right">
        <Button
          variant="destructive"
          on:click={() =>
            toast("Server has been started", {
              description: "Sunday, December 03, 2023 at 9:00 AM",
            })}
        >
          Stop server
        </Button>
        <Button variant="secondary">Admin</Button>
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
      </div>
    </div>
  </div>
  <br />
  <Textarea
    wrap="off"
    class="logcat"
    style="min-height: 200px; max-height: 200px; cursor: default; font-family: 'Source Code Pro', monospace;"
    disabled
    placeholder="05:56:12 [main] Initialising system Initialising system Initialising systemalpha beta and gamma
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising system alpha Initialising Initialising system system beta and gamma...
05:56:12 [main] Windows version []] alphaInitialising Initialising system system beta and deya...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] panel system alpha beta andInitialising system gamma...
05:56:12 [main] Initialising system alphaInitialising system  KInitialising system ADAS and gamma...
05:56:12 [main] Initialising system alphaInitialising Initialising system system beta and 454r...
05:56:12 [main] Comntrol system alpha betInitialising Initialising system systema and gamma...
05:56:12 [main] Initialising system alphaInitialising system beta and gamma...
05:56:12 [main] Preqrassd system dadd beInitialising Initialising system systemta and gamma...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
05:56:12 [main] Initialising JAMP alcekiInitialising Initialising system Initialising system system ng pha beta ...
"
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
  :global(.logcat) {
    -ms-overflow-style: none; /* for Internet Explorer, Edge */
    scrollbar-width: none; /* for Firefox */
    overflow-y: scroll;
  }
  :global(.logcat::-webkit-scrollbar) {
    display: none; /* for Chrome, Safari, and Opera */
  }
</style>
