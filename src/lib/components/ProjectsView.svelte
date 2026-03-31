<script lang="ts">
	import type { Project } from '$lib/types/analysis';
	import { activeProject, activeTab } from '$lib/stores';
	import {
		listProjects, createProject, updateProject, deleteProject, setActiveProject
	} from '$lib/api';
	import type { ProjectSummary } from '$lib/types';
	import ProjectsHeader from '$lib/components/projects/ProjectsHeader.svelte';
	import ProjectGrid from '$lib/components/projects/ProjectGrid.svelte';
	import ProjectFormModal from '$lib/components/projects/ProjectFormModal.svelte';
	import DeleteProjectModal from '$lib/components/projects/DeleteProjectModal.svelte';

	let projects = $state<ProjectSummary[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// Form state
	let showForm = $state(false);
	let editingProject = $state<Project | null>(null);
	let formName = $state('');
	let formClient = $state('');
	let formSite = $state('');
	let formAssessor = $state('');
	let formStart = $state('');
	let formEnd = $state('');
	let formNotes = $state('');
	let formError = $state<string | null>(null);
	let formSaving = $state(false);

	// Delete confirmation
	let confirmDeleteId = $state<number | null>(null);

	async function load() {
		loading = true;
		error = null;
		try {
			projects = await listProjects();
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	function openNewForm() {
		editingProject = null;
		formName = '';
		formClient = '';
		formSite = '';
		formAssessor = '';
		formStart = '';
		formEnd = '';
		formNotes = '';
		formError = null;
		showForm = true;
	}

	function openEditForm(summary: ProjectSummary) {
		// Pre-fill from summary (full Project fields are superset of summary)
		editingProject = summary as unknown as Project;
		formName = summary.name;
		formClient = summary.client_name;
		formSite = summary.site_name;
		formAssessor = '';
		formStart = '';
		formEnd = '';
		formNotes = '';
		formError = null;
		showForm = true;
	}

	function closeForm() {
		showForm = false;
	}

	async function handleSave() {
		if (!formName.trim()) {
			formError = 'Project name is required.';
			return;
		}
		formSaving = true;
		formError = null;
		try {
			if (editingProject) {
				await updateProject(
					editingProject.id, formName,
					formClient, formSite, formAssessor,
					formStart, formEnd, formNotes
				);
			} else {
				await createProject(
					formName, formClient, formSite, formAssessor,
					formStart, formEnd, formNotes
				);
			}
			showForm = false;
			await load();
		} catch (e) {
			formError = String(e);
		} finally {
			formSaving = false;
		}
	}

	async function handleOpen(summary: ProjectSummary) {
		try {
			const project = await setActiveProject(summary.id);
			activeProject.set(project);
			activeTab.set('topology');
		} catch (e) {
			error = String(e);
		}
	}

	async function handleDelete(id: number) {
		try {
			await deleteProject(id);
			confirmDeleteId = null;
			await load();
		} catch (e) {
			error = String(e);
		}
	}

	function formatDate(iso: string): string {
		if (!iso) return '—';
		try {
			return new Date(iso).toLocaleDateString(undefined, {
				year: 'numeric', month: 'short', day: 'numeric'
			});
		} catch {
			return iso;
		}
	}

	// Load on mount
	$effect(() => {
		load();
	});
</script>

<div class="projects-view">
	<ProjectsHeader onCreate={openNewForm} />

	{#if error}
		<div class="error-bar">{error}</div>
	{/if}

	<ProjectGrid
		{loading}
		{projects}
		{formatDate}
		onCreate={openNewForm}
		onOpen={handleOpen}
		onEdit={openEditForm}
		onRequestDelete={(id) => (confirmDeleteId = id)}
	/>
</div>

<ProjectFormModal
	show={showForm}
	editing={editingProject !== null}
	{formError}
	{formSaving}
	bind:projectName={formName}
	bind:clientName={formClient}
	bind:siteName={formSite}
	bind:assessorName={formAssessor}
	bind:startDate={formStart}
	bind:endDate={formEnd}
	bind:notes={formNotes}
	onClose={closeForm}
	onSave={handleSave}
/>

<DeleteProjectModal
	show={confirmDeleteId !== null}
	onCancel={() => {
		confirmDeleteId = null;
	}}
	onConfirm={() => {
		if (confirmDeleteId !== null) {
			return handleDelete(confirmDeleteId);
		}
	}}
/>

<style>
	.projects-view {
		padding: 32px 40px;
		height: 100%;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* ── Error ──────────────────────────────── */

	.error-bar {
		padding: 10px 14px;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 4px;
		color: #ef4444;
		font-size: 12px;
	}

</style>
