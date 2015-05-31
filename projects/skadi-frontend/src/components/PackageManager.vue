<script setup lang="ts">

</script>

<template>
  <div class="package-manager">
    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="container">
        <nav class="home-navigation">
          <a href="#" class="nav-link active">首页</a>
          <a href="#" class="nav-link">浏览包</a>
          <a href="#" class="nav-link">文档</a>
          <a href="#" class="nav-link">关于</a>
        </nav>
        <div class="search-bar">
          <input type="text" placeholder="搜索包..." class="search-input" />
          <button class="search-button">搜索</button>
        </div>
        <div class="auth-buttons">
          <button class="login-button">登录</button>
        </div>
      </div>
    </header>

    <!-- 主要内容区域 -->
    <main class="main">
      <div class="container">
        <!-- 统计信息和发布按钮 -->
        <section class="stats-section">
          <div class="stats-container">
            <div class="stat-item">
              <h3>1,234,567</h3>
              <p>总下载量</p>
            </div>
            <div class="stat-item">
              <h3>8,901</h3>
              <p>包总数</p>
            </div>
          </div>
          <button class="publish-button">发布包</button>
        </section>

        <!-- 包列表区域 -->
        <div class="packages-grid">
          <!-- 精选包 -->
          <section class="package-section">
            <h2>精选包</h2>
            <div class="package-cards">
              <div class="package-card" v-for="i in 3" :key="i">
                <h3>示例包名称</h3>
                <p class="version">v1.0.0</p>
                <p class="description">这是一个示例包的简要描述，展示包的主要功能和用途。</p>
              </div>
            </div>
            <div class="view-more">
              <button class="view-more-button">查看更多</button>
            </div>
          </section>

          <!-- 最近更新 -->
          <section class="package-section">
            <h2>最近更新</h2>
            <div class="package-cards">
              <div class="package-card" v-for="i in 3" :key="i">
                <h3>更新包名称</h3>
                <p class="version">v2.1.0</p>
                <p class="description">最近更新的包描述，展示包的更新内容和改进。</p>
              </div>
            </div>
            <div class="view-more">
              <button class="view-more-button">查看更多</button>
            </div>
          </section>

          <!-- 新发布 -->
          <section class="package-section">
            <h2>新发布</h2>
            <div class="package-cards">
              <div class="package-card" v-for="i in 3" :key="i">
                <h3>新包名称</h3>
                <p class="version">v0.1.0</p>
                <p class="description">新发布的包描述，展示包的特点和创新之处。</p>
              </div>
            </div>
            <div class="view-more">
              <button class="view-more-button">查看更多</button>
            </div>
          </section>
        </div>
      </div>
    </main>
  </div>
</template>

<style lang="scss" scoped>
.package-manager {
  min-height: 100vh;
  background-color: #f8f9fa;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
}

.header {
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  padding: 1rem 0;

  .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 2rem;
  }
}

.home-navigation {
  display: flex;
  gap: 1.5rem;

  .nav-link {
    color: #495057;
    text-decoration: none;
    font-weight: 500;
    padding: 0.5rem 0;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;

    &:hover,
    &.active {
      color: #4dabf7;
      border-bottom-color: #4dabf7;
    }
  }
}

.search-bar {
  display: flex;
  gap: 0.5rem;
  flex: 1;
  max-width: 400px;

  .search-input {
    flex: 1;
    padding: 0.5rem 1rem;
    border: 2px solid #e9ecef;
    border-radius: 4px;
    font-size: 1rem;

    &:focus {
      outline: none;
      border-color: #4dabf7;
    }
  }

  .search-button {
    padding: 0.5rem 1rem;
    background-color: #4dabf7;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;

    &:hover {
      background-color: #339af0;
    }
  }
}

.auth-buttons {
  .login-button {
    padding: 0.5rem 1rem;
    background-color: transparent;
    border: 2px solid #4dabf7;
    color: #4dabf7;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      background-color: #4dabf7;
      color: white;
    }
  }
}

.main {
  padding: 2rem 0;
}

.stats-section {
  background-color: white;
  border-radius: 8px;
  padding: 2rem;
  margin-bottom: 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

  .stats-container {
    display: flex;
    gap: 3rem;
  }

  .stat-item {
    h3 {
      font-size: 2rem;
      color: #4dabf7;
      margin: 0;
    }

    p {
      margin: 0.5rem 0 0;
      color: #6c757d;
    }
  }

  .publish-button {
    padding: 0.75rem 1.5rem;
    background-color: #40c057;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;

    &:hover {
      background-color: #37b24d;
    }
  }
}

.packages-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2rem;
}

.package-section {
  h2 {
    margin: 0 0 1rem;
    color: #343a40;
  }

  .package-cards {
    display: grid;
    gap: 1rem;
  }

  .package-card {
    background-color: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s;

    &:hover {
      transform: translateY(-2px);
    }

    h3 {
      margin: 0;
      color: #4dabf7;
    }

    .version {
      color: #6c757d;
      font-size: 0.875rem;
      margin: 0.5rem 0;
    }

    .description {
      color: #495057;
      margin: 0;
      line-height: 1.5;
    }
  }

  .view-more {
    margin-top: 1.5rem;
    text-align: center;

    .view-more-button {
      padding: 0.5rem 1rem;
      background-color: transparent;
      border: 2px solid #4dabf7;
      color: #4dabf7;
      border-radius: 4px;
      cursor: pointer;
      transition: all 0.2s;

      &:hover {
        background-color: #4dabf7;
        color: white;
      }
    }
  }
}
</style>