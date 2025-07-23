<script setup>
import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api";

const students = ref([]);
const form = reactive({
  id: "",
  name: "",
  math: "",
  english: "",
  physics: "",
});
const editing = ref(false);
const errorMsg = ref("");

async function fetchStudents() {
  students.value = await invoke("get_students");
}

function resetForm() {
  form.id = "";
  form.name = "";
  form.math = "";
  form.english = "";
  form.physics = "";
  editing.value = false;
  errorMsg.value = "";
}

async function addStudent() {
  errorMsg.value = "";
  try {
    await invoke("add_student", {
      student: {
        id: form.id,
        name: form.name,
        math: parseFloat(form.math),
        english: parseFloat(form.english),
        physics: parseFloat(form.physics),
        average: 0,
        rank: 0,
      },
    });
    await fetchStudents();
    resetForm();
  } catch (e) {
    errorMsg.value = String(e);
  }
}

async function deleteStudent(id) {
  await invoke("delete_student", { id });
  await fetchStudents();
}

function editStudent(student) {
  form.id = student.id;
  form.name = student.name;
  form.math = student.math;
  form.english = student.english;
  form.physics = student.physics;
  editing.value = true;
  errorMsg.value = "";
}

async function updateStudent() {
  errorMsg.value = "";
  try {
    await invoke("update_student", {
      student: {
        id: form.id,
        name: form.name,
        math: parseFloat(form.math),
        english: parseFloat(form.english),
        physics: parseFloat(form.physics),
        average: 0,
        rank: 0,
      },
    });
    await fetchStudents();
    resetForm();
  } catch (e) {
    errorMsg.value = String(e);
  }
}

onMounted(fetchStudents);
</script>

<template>
  <main class="container">
    <h1>学生成绩管理系统</h1>
    <form class="form" @submit.prevent="editing ? updateStudent() : addStudent()">
      <input v-model="form.id" :readonly="editing" placeholder="学号" required />
      <input v-model="form.name" placeholder="姓名" required />
      <input v-model="form.math" type="number" min="0" max="100" placeholder="高等数学" required />
      <input v-model="form.english" type="number" min="0" max="100" placeholder="英语" required />
      <input v-model="form.physics" type="number" min="0" max="100" placeholder="物理" required />
      <button type="submit">{{ editing ? '修改' : '添加' }}</button>
      <button type="button" @click="resetForm" v-if="editing">取消</button>
    </form>
    <p v-if="errorMsg" style="color:red">{{ errorMsg }}</p>
    <table class="table">
      <thead>
        <tr>
          <th>学号</th>
          <th>姓名</th>
          <th>高等数学</th>
          <th>英语</th>
          <th>物理</th>
          <th>平均成绩</th>
          <th>名次</th>
          <th>操作</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="stu in students" :key="stu.id">
          <td>{{ stu.id }}</td>
          <td>{{ stu.name }}</td>
          <td>{{ stu.math }}</td>
          <td>{{ stu.english }}</td>
          <td>{{ stu.physics }}</td>
          <td>{{ stu.average.toFixed(2) }}</td>
          <td>{{ stu.rank }}</td>
          <td>
            <button @click="editStudent(stu)">编辑</button>
            <button @click="deleteStudent(stu.id)">删除</button>
          </td>
        </tr>
      </tbody>
    </table>
  </main>
</template>

<style scoped>
.container {
  max-width: 900px;
  margin: 0 auto;
  padding: 2em;
}
h1 {
  text-align: center;
}
.form {
  display: flex;
  gap: 0.5em;
  margin-bottom: 1em;
  flex-wrap: wrap;
  justify-content: center;
}
.form input {
  padding: 0.5em;
  border-radius: 4px;
  border: 1px solid #ccc;
  width: 120px;
}
.form button {
  padding: 0.5em 1em;
  border-radius: 4px;
  border: none;
  background: #396cd8;
  color: #fff;
  cursor: pointer;
}
.form button[type="button"] {
  background: #aaa;
}
.table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1em;
}
.table th, .table td {
  border: 1px solid #ddd;
  padding: 0.5em 0.7em;
  text-align: center;
}
.table th {
  background: #f0f0f0;
}
.table tr:nth-child(even) {
  background: #fafafa;
}
</style>
