In the [previous section](/docs/graph-config), we explored the **initial configuration options** for a graph and how they can influence its behavior.

<br />

Now, let’s put those settings into practice by building a simple yet interactive example: a graph that continuously **adds up the numbers entered by the user**.

<br />

This example will make use of:

* **Input data type** → to ensure the graph correctly interprets the user’s input as numbers.
* **Initial context** → to store the running total.
* **Preserve context** → so the sum persists across multiple inputs during execution.

<br />

By the end of this exercise, you’ll have a graph that can keep track of an accumulated sum while interacting with the user in real time.

<br />

You can open the <a href="/builder" title="Open Graph Builder in new tab" target="_blank">Graph Builder</a> to start working on our new example.

<br />

If you have previously been working on another graph, the builder will automatically restore your **last saved configuration**.
To start from scratch, open the **"Options"** dropdown menu and select **"New graph"**.
This will clear the previous configuration and display only the three initial nodes.

<img src="/img/docs/dropdown_new_graph.png" alt='Menu option "New graph"' class="h-20 border border-gray-200 rounded-lg shadow-sm dark:border-gray-700 block mt-2 mb-2" />

---

## Initial Configuration for the Number-Summing Graph

Now let’s set up the **initial configuration** for our graph.

1. Open the **"Options"** dropdown menu and click **"Graph data"**.
2. In the **"Graph config"** section:

   * **Input data type** → Select **`Number`**.
   * **Initial context** → Enter the following JSON:

     <code>
     { "number": 0 }
     </code>

     This field will store the running total as the user enters numbers.
     We initialize it with zero so that the first user input remains unchanged when added.
   * **Preserve context** → Check this box. This ensures the context is kept between executions, allowing the sum to grow with each new input.

<img src="/img/docs/graph_config_add_numbers.png" alt='Sum example graph configuration' class="h-54 border border-gray-200 rounded-lg shadow-sm dark:border-gray-700 block mt-2 mb-2" />

---

## Renaming the Initial Node

Now let’s move to the initial node.
To do this, open the **"Options"** menu and select **"Graph view"**.

<br />

Click on the initial node — this is the entry point of the graph and, by default, its **Id** is set to **"Entry node"**.

<br />

The **Id** field determines the label you see on the node in the graph view.
You can change this to any descriptive name you like.

<br />

In our case, since this node will be responsible for performing the sum, we’ll rename it to:

<br />

**`Sum numbers`**

<br />

Keep in mind that each node in a graph must have a unique **Id**.
If two nodes share the same **Id**, the graph will not work correctly.

<img src="/img/docs/node_id.png" alt='Field node id' class="h-40 border border-gray-200 rounded-lg shadow-sm dark:border-gray-700 block mt-2 mb-2" />

---

## Configuring the Node to Perform the Sum

Next, let’s configure our initial node so it can add the numbers entered by the user.

<br />

First, locate the section in the form labeled **"Node executor"**.

<br />

In the **"Executor type"** field, select **`ContextMut`**.  
This executor type allows us to modify the graph’s context during execution.

<br />

When you select this option, a button labeled **"Add ContextMut"** will appear.  
Click it, and a new configuration section will be added.

<br />

This new section contains three main parts:

1. **From** – This is a **DataFrom**, just like the one we saw in the chat example. As we mentioned before, this structure appears in many parts of the builder.
   Here, you can retrieve and manipulate data from either the graph’s context or its input.

2. **To** – This specifies the path in the context where the retrieved or processed data should be stored.

3. **Condition** – A field where you can define a condition to control execution.
   If the condition evaluates to `true`, the **From** and **To** sections will run; otherwise, they will be skipped.
   For this example, we’ll leave the default value **`True`**, meaning the condition will always pass.

---

### Configuring the **DataFrom** for Addition

Inside the **From** section, set the **Type** to **`Operation`**.  
When you do this, a dropdown will appear allowing you to select the operation to perform.
Choose **`Add`**.

<br />

Selecting **Add** will create two additional **DataFrom** sections beneath it.  
These represent the two values we want to sum.

* **First DataFrom**:

  * Set **Type** to **`ParsedInput`**.
    This retrieves the user’s input as a number.
    (Recall that in the graph’s initial configuration, we set **Input data type** to **Number** so the input is automatically converted.)
  * In the **Path** field, we can leave it blank or set it to `/` since we are dealing with a single number.
    If we were working with an object or array, we would specify the exact path here.

* **Second DataFrom**:

  * Set **Type** to **`Context`**.
    This allows us to retrieve data from the graph’s context.
  * In **Path**, enter `number`, which is where we stored the running total in the initial configuration.

---

### Storing the Result

In the **To** section:

* **Path**: `number` (so the result is stored in the same field we’ve been using).
* **Type**: `Number` (to store it as a numeric value; leaving this empty would store it as a string).
* **Merge**: Leave it empty or set it to **Insert** (the default).
  This ensures that if the path exists, the value is replaced, which is exactly what we want since we’re updating the running total.

---

<br />

With this setup, our node will take the user’s input, add it to the current value stored in `number`, and save the new total back into the context.

---

## Configuring the Graph Output

The final step in our example is to configure the graph’s output.

### Connecting the Initial Node to “Exit Ok”

First, let’s connect our initial node to the “Exit Ok” node. We’ll follow the same process as in the **chat example**:

1. Open the **Options** menu and click **Graph view**.
2. Click the **Start edit mode** button.
3. Click on the initial node (which we renamed to **Sum numbers**).
4. Without releasing the mouse, drag the connection line to the **Exit Ok** node.
5. Release the mouse to create a connection (arrow) between the two nodes.
6. Finally, click the **End edit mode** button to prevent creating new connections by mistake.

### Adding Output Text to the Graph Exit

Now let’s add a custom message to the graph’s output:

1. Click on the arrow connecting **Sum numbers** and **Exit Ok**.

2. A form will appear. The first field is **Next step**, which determines which node the flow continues to. In our case, it’s already set to **Exit Ok**, so we leave it as is.

3. Next to it, you’ll see a button labeled **Add exit text**. Click it to open a new section with three parts:

   * **Prefix** — a text field for adding content before the dynamic output.
   * **DataFrom** — where you select data from the graph’s context or input.
   * **Suffix** — a text field for adding content after the dynamic output.

4. In **Prefix**, enter: `Result: `

   (Note the space at the end so the result doesn’t stick to the text.)

5. In the **DataFrom** section, select **Context** and set the path to `number` — this is where we stored the sum result.

6. In **Suffix**, enter something like:

   `

   Please enter another number to add.
   `

   (The leading line break ensures it doesn’t appear glued to the result text.)

With this configuration, when the graph runs, the output will display something like:

<br />

<code>
Result: 15  <br />
Please enter another number to add.
</code>

<br />
<br />

You can press **Add exit text** again to add more sections — the final output will concatenate all of them. In this simple example, we only need one, but in more complex graphs this feature can be quite useful.

---

### Running the Graph

We can run our graph using the <a href="https://github.com/afuentesan/awpak-tui/tree/main/awpak-ai-cmd-client" target="_blank" title="Github of awpak-ai-cmd-client command-line tool">awpak-ai-cmd-client</a> application, just like we did in the <a href="/docs/getting-started" target="_blank" title="Chat example in getting started guide">chat example</a>.  
To do so, execute the following command:

<br />

`awpak-ai-cmd-client --path="/path_to_json_graph.json" --chat`

<br />

In this case, we don’t need to specify the `--trace` option because the graph output is displayed by default — and that’s exactly what we want to see here.

<br />

The result of running this command looks like this:

![Graph execution showing the sum results](/img/docs/example_sum_number.png "Example of executing the sum graph")

---


