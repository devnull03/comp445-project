import random
import csv


def load_test_data() -> list[str]:
    """
    Load a list of test documents. We're using the testing documents given in the assignment,
    but this function can be modified to fetch or load the document in any way, it just needs
    to return it as a list of strings.
    """
    documents = [
        "This is document 1 about cats and dogs.",
        "Document 2 talks about dogs and their behavior.",
        "The third document is about cats and their habits.",
        "Document number 4 discusses different dog breeds.",
        "Document 5 covers cat breeds and their characteristics.",
        "Document 6 is all about training your dog.",
        "In document 7, we explore the world of cat care.",
        "Document 8 delves into the history of domesticated dogs.",
        "The ninth document discusses feline health issues.",
        "The last document, number 10, is about dog nutrition and diet.",
        "Document 11 covers the topic of cat allergies.",
        "In document 12, we talk about dog sports and activities.",
        "The 13th document is about the history and origins of cats.",
        "Document 14 discusses famous cats in pop culture.",
        "Document 15 is all about dog training techniques.",
    ]
    return documents


def load_data(file_path: str) -> dict[int, dict]:
    """
    Load data from a CSV file and store it in a dictionary with headings.
    """
    records = {}
    with open(file_path, mode="r", encoding="utf-8") as file:
        reader = csv.DictReader(file)
        for idx, row in enumerate(reader):
            records[idx] = row
    return records


def create_shingles(data: str, k: int) -> set[str]:
    """
    Create a set of k-shingles from the input string.
    """
    shingle_set = set()
    for i in range(len(data) - k + 1):
        shingle_set.add(data[i : i + k])
    return shingle_set


def create_shingled_dataset(data: list[str], k: int) -> dict[int, set[str]]:
    """
    Create a dataset of k-shingled documents.
    """
    return {i: create_shingles(data[i], k) for i in range(len(data))}


def create_hash_func() -> callable:
    """
    Create a random hash function.
    """
    a, b = random.randint(10, 10_000), random.randint(10, 10_000)
    large_prime = 95633
    hash_base = 7
    rebase = lambda s: sum(
        map(lambda b: int(b[1]) * hash_base ** (b[0]), enumerate(bin(ord(s))[2:]))
    )
    return lambda s: (a * sum(map(rebase, s)) + b) % large_prime


def generate_hash_funcs(k: int) -> list[callable]:
    """
    Generate a list of k random hash functions.
    """
    return [create_hash_func() for _ in range(k)]


def generate_minhash_signature(
    data: set[str], hash_func_list: list[callable]
) -> list[int]:
    """
    Generate a MinHash signature for a set of shingles.
    """
    minhash_signature = []

    for hash_func in hash_func_list:
        min_value = float("inf")
        for shingle in data:
            hash_value = hash_func(shingle)
            min_value = min(min_value, hash_value)
        minhash_signature.append(min_value)

    return minhash_signature


def jaccard(a: set, b: set) -> float:
    """
    Calculate the Jaccard similarity between two sets.
    """
    return len(a.intersection(b)) / len(a.union(b))


def minhash_similarity(a: list[int], b: list[int]) -> float:
    """
    Calculate the similarity between two MinHash signatures.
    (just for some extra testing)
    """
    matches = sum(1 for x, y in zip(a, b) if x == y)
    return matches / len(a)


if __name__ == "__main__":

    # Part 1: MinHashing ---------------------------------------------------------
    shingle_length = 4
    signature_len = 50

    # data = load_test_data()
    data = load_data(
        "/home/devnull03/school/COMP455/project/server/src/bin/evaluation.csv"
    )
    shingled_dataset = create_shingled_dataset(list(data.values()), shingle_length)

    hash_functions = generate_hash_funcs(signature_len)
    minhashed_dataset = {}

    for doc_id, shingles in shingled_dataset.items():
        minhash_signature = generate_minhash_signature(shingles, hash_functions)
        minhashed_dataset[doc_id] = minhash_signature
        # print(f"Document {doc_id} Minhash Signature: {minhash_signature}")

    # ----------------------------------------------------------------------------

    print()

    # Part 2: Jaccard Similarities -----------------------------------------------
    similarity_threshold = 0.3
    similarities = {}
    for doc1_id, doc1 in minhashed_dataset.items():
        similarities[doc1_id] = {}
        for doc2_id, doc2 in minhashed_dataset.items():
            #     e = jaccard(set(doc1), set(doc2))
            e = minhash_similarity(doc1, doc2)  # gives better similarity ratings ?

            if e >= similarity_threshold and (doc1_id != doc2_id):
                similarities[doc1_id][doc2_id] = e

        # print(similarities[doc1_id])

    # Save similarities to a file
    with open("similarities.csv", mode="w", encoding="utf-8") as file:
        writer = csv.writer(file)
        writer.writerow(["Document 1", "Document 2", "Similarity"])
        for doc1_id, doc_similarities in similarities.items():
            for doc2_id, similarity in doc_similarities.items():
                writer.writerow([doc1_id, doc2_id, similarity])

    # ----------------------------------------------------------------------------


