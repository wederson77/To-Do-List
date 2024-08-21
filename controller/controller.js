 const apiUrl = 'http://localhost:9000/todos';

// Buscar tarefas do backend
async function fetchTasks() {
    try {
        const response = await fetch(apiUrl);
        if (!response.ok) {
            throw new Error('Erro ao buscar as tarefas');
        }
        const tasks = await response.json();
        renderTasks(tasks);
    } catch (error) {
        console.error('Erro:', error);
    }
}

// Renderizar tarefas
function renderTasks(tasks) {
    const taskList = document.getElementById('taskList');
    taskList.innerHTML = ''; // Limpar lista antes de renderizar
    tasks.forEach(task => {
        const listItem = document.createElement('li');
        const taskTitle = document.createElement('span');
        taskTitle.textContent = task.title;
        if (task.completed) {
            taskTitle.classList.add('completed');
        }
        listItem.appendChild(taskTitle);

        // Botão de editar
        const editButton = document.createElement('button');
        editButton.textContent = 'Editar';
        editButton.onclick = () => editTask(task.id, task.title);
        listItem.appendChild(editButton);

        // Botão de excluir
        const deleteButton = document.createElement('button');
        deleteButton.id = 'delete-taks';
        deleteButton.textContent = 'Excluir';
        deleteButton.onclick = () => deleteTask(task.id);
        listItem.appendChild(deleteButton);

        taskList.appendChild(listItem);
    });
}

// Adicionar tarefa
async function addTask(title) {
    try {
        const response = await fetch(apiUrl, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ title, completed: false })
        });
        if (!response.ok) {
            throw new Error('Erro ao adicionar a tarefa');
        }
        fetchTasks();
    } catch (error) {
        console.error('Erro:', error);
    }
}

// Editar tarefa
function showConfirmModal() {
    return new Promise((resolve) => {
        const modal = document.getElementById('confirmModal');
        const confirmBtn = document.getElementById('confirmBtn');
        const cancelBtn = document.getElementById('cancelBtn');
        
        modal.style.display = 'block';

        confirmBtn.onclick = function() {
            modal.style.display = 'none';
            resolve(true); // Retorna true se o usuário confirmar
        };

        cancelBtn.onclick = function() {
            modal.style.display = 'none';
            resolve(false); // Retorna false se o usuário cancelar
        };

        window.onclick = function(event) {
            if (event.target == modal) {
                modal.style.display = 'none';
                resolve(false);
            }
        };
    });
}

async function editTask(id, oldTitle) {
    const completed = await showConfirmModal(); // Exibe o modal customizado e espera pela resposta

    if (completed !== null) {
        try {
            const response = await fetch(`http://localhost:9000/todos/${id}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ completed })
            });

            if (!response.ok) {
                throw new Error('Erro ao editar a tarefa');
            }

            //const updatedTask = await response.json(); // Obtém a tarefa atualizada do servidor

            // Atualiza a tarefa localmente na interface
            //updateTaskUI(updatedTask);

            console.log('Tarefa editada com sucesso!');
        } catch (error) {
            console.log('Erro:', error);
        }
    }

    fetchTasks()
}

// Excluir tarefa
async function deleteTask(id) {
    try {
        const response = await fetch(`${apiUrl}/${id}`, {
            method: 'DELETE'
        });
        if (!response.ok) {
            throw new Error('Erro ao excluir a tarefa');
        }
        fetchTasks();
    } catch (error) {
        console.error('Erro:', error);
    }
}

// Manipular o envio do formulário para adicionar uma nova tarefa
document.getElementById('taskForm').addEventListener('submit', function(event) {
    event.preventDefault();
    const newTaskTitle = document.getElementById('newTaskTitle').value;
    if (newTaskTitle) {
        addTask(newTaskTitle);
        document.getElementById('newTaskTitle').value = ''; // Limpar o campo de entrada
    }
});

// Carregar as tarefas ao iniciar
fetchTasks();