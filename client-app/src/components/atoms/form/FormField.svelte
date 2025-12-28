<script lang="ts">
  type InputType = "text" | "email" | "password" | "number" | "tel" | "url" | "search";

  interface Props {
    id: string;
    label: string;
    type?: InputType;
    value?: string | number;
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
    readonly?: boolean;
    error?: string;
    helpText?: string;
    class?: string;
  }

  let {
    id,
    label,
    type = "text",
    value = $bindable(""),
    placeholder = "",
    required = false,
    disabled = false,
    readonly = false,
    error = "",
    helpText = "",
    class: className = "",
  }: Props = $props();
</script>

<div class="mb-3 {className}">
  <label class="form-label" for={id}>
    {label}
    {#if required}
      <span class="text-danger">*</span>
    {/if}
  </label>
  <input
    {id}
    {type}
    class="form-control"
    class:is-invalid={!!error}
    bind:value
    {placeholder}
    {required}
    {disabled}
    {readonly}
  />
  {#if error}
    <div class="invalid-feedback">{error}</div>
  {:else if helpText}
    <div class="form-text">{helpText}</div>
  {/if}
</div>
