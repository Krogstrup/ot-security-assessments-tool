<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { signatureSummary } from '$lib/stores';
	import { getSignatures, reloadSignatures, testSignature } from '$lib/api';
	import type { SignatureTestResult } from '$lib/types';
	import SignatureToolbar from './signature/SignatureToolbar.svelte';
	import SignatureListPanel from './signature/SignatureListPanel.svelte';
	import SignatureTestResults from './signature/SignatureTestResults.svelte';
	import { defaultSignatureYaml, signatureToYaml } from './signature/signatureYaml';

	let editorContainer: HTMLDivElement;
	let editorView: any = null;

	let testResult = $state<SignatureTestResult | null>(null);
	let testError = $state<string | null>(null);
	let testing = $state(false);
	let reloading = $state(false);
	let selectedSignature = $state<string | null>(null);

	async function initEditor() {
		const { EditorView, basicSetup } = await import('codemirror');
		const { EditorState } = await import('@codemirror/state');
		const { yaml } = await import('@codemirror/lang-yaml');
		const { oneDark } = await import('@codemirror/theme-one-dark');

		const state = EditorState.create({
			doc: defaultSignatureYaml,
			extensions: [basicSetup, yaml(), oneDark, EditorView.lineWrapping]
		});

		editorView = new EditorView({
			state,
			parent: editorContainer
		});
	}

	function getEditorContent(): string {
		if (!editorView) return '';
		return editorView.state.doc.toString();
	}

	function setEditorContent(content: string) {
		if (!editorView) return;
		editorView.dispatch({
			changes: {
				from: 0,
				to: editorView.state.doc.length,
				insert: content
			}
		});
	}

	async function handleTest() {
		testing = true;
		testError = null;
		testResult = null;
		try {
			const yaml = getEditorContent();
			if (!yaml.trim()) {
				testError = 'Editor is empty';
				return;
			}
			testResult = await testSignature(yaml);
		} catch (err: any) {
			testError = err.toString();
		} finally {
			testing = false;
		}
	}

	async function handleReload() {
		reloading = true;
		try {
			await reloadSignatures();
			const summary = await getSignatures();
			signatureSummary.set(summary);
			testError = null;
			testResult = null;
		} catch (err: any) {
			testError = `Reload failed: ${err}`;
		} finally {
			reloading = false;
		}
	}

	async function loadSignatures() {
		try {
			const summary = await getSignatures();
			signatureSummary.set(summary);
		} catch (err) {
			console.warn('Failed to load signatures:', err);
		}
	}

	function selectSignature(name: string) {
		selectedSignature = name;
		const sig = $signatureSummary.signatures.find((s) => s.name === name);
		if (sig) {
			setEditorContent(signatureToYaml(sig));
		}
	}

	onMount(async () => {
		await initEditor();
		await loadSignatures();
	});

	onDestroy(() => {
		editorView?.destroy();
	});
</script>

<div class="sig-editor-container">
	<SignatureToolbar
		totalCount={$signatureSummary.total_count}
		{reloading}
		{testing}
		onReload={handleReload}
		onTest={handleTest}
	/>

	<div class="sig-content">
		<SignatureListPanel
			signatures={$signatureSummary.signatures}
			{selectedSignature}
			onSelect={selectSignature}
		/>

		<div class="sig-editor-panel">
			<div class="editor-wrapper" bind:this={editorContainer}></div>
			<SignatureTestResults {testError} {testResult} />
		</div>
	</div>
</div>

<style>
	.sig-editor-container {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.sig-content {
		display: flex;
		flex: 1;
		min-height: 0;
		overflow: hidden;
	}

	.sig-editor-panel {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
		overflow: hidden;
	}

	.editor-wrapper {
		flex: 1;
		min-height: 200px;
		overflow: auto;
	}

	.editor-wrapper :global(.cm-editor) {
		height: 100%;
		font-size: 12px;
	}

	.editor-wrapper :global(.cm-scroller) {
		overflow: auto;
	}
</style>
