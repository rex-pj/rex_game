<script lang="ts">
  interface Props {
    id: string;
    label: string;
    value?: string;
    placeholder?: string;
    rows?: number;
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
    value = $bindable(""),
    placeholder = "",
    rows = 3,
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
  <textarea
    {id}
    class="form-control"
    class:is-invalid={!!error}
    bind:value
    {placeholder}
    {rows}
    {required}
    {disabled}
    {readonly}
  ></textarea>
  {#if error}
    <div class="invalid-feedback">{error}</div>
  {:else if helpText}
    <div class="form-text">{helpText}</div>
  {/if}
</div>
